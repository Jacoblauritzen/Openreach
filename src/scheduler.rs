//! Task-slot creation — the only module that inserts `Task` rows. (`core/scheduler.py`)
//!
//! Drains (`find_email`/`email`/`follow_up`) are lazy capacity tokens minted when
//! there is eligible work under the day's send cap; bound polls (`collect_email`)
//! are one persisted row per in-flight lookup, chained poll→poll.

use anyhow::Result;
use chrono::{Duration, Utc};
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};

use crate::models::{Mailbox, SiteConfig, Task, TaskType};
use crate::session::OperatorSession;

/// Is a drain slot of this type already pending for the campaign? (`_has_pending`)
async fn has_pending(db: &SqlitePool, task_type: TaskType, campaign_id: i64) -> Result<bool> {
    let row = sqlx::query(
        "SELECT 1 FROM core_task
         WHERE task_type = ?1 AND status = 'pending'
           AND json_extract(payload, '$.campaign_id') = ?2 LIMIT 1",
    )
    .bind(task_type.as_str())
    .bind(campaign_id)
    .fetch_optional(db)
    .await?;
    Ok(row.is_some())
}

/// Bulk-create `n` lazy drain slots (payload = campaign_id only), scheduled now.
async fn create_lazy_slots(
    db: &SqlitePool,
    task_type: TaskType,
    campaign_id: i64,
    n: i64,
) -> Result<i64> {
    let now = Utc::now();
    let payload = json!({ "campaign_id": campaign_id });
    for _ in 0..n {
        Task::create(db, task_type, now, &payload).await?;
    }
    Ok(n)
}

async fn count_scalar(db: &SqlitePool, sql: &str, campaign_id: i64) -> Result<i64> {
    let row = sqlx::query(sql).bind(campaign_id).fetch_one(db).await?;
    Ok(row.get::<i64, _>(0))
}

/// Mint one `find_email` submit slot when there's send-headroom for its result
/// today. (`flush_find_email_queue`)
pub async fn flush_find_email_queue(
    db: &SqlitePool,
    campaign_id: i64,
) -> Result<i64> {
    if has_pending(db, TaskType::FindEmail, campaign_id).await? {
        return Ok(0);
    }
    let cfg = SiteConfig::load(db).await?;
    if !Mailbox::any_exists(db).await? || !cfg.bettercontact_configured() {
        return Ok(0);
    }
    let remaining = Mailbox::remaining_today(db).await?;
    let in_pipeline = count_scalar(
        db,
        "SELECT COUNT(*) FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
         WHERE d.campaign_id = ?1 AND d.state IN ('Ready to Email', 'Finding Email')
           AND l.disqualified = 0",
        campaign_id,
    )
    .await?;
    if in_pipeline >= remaining {
        return Ok(0);
    }
    create_lazy_slots(db, TaskType::FindEmail, campaign_id, 1).await?;
    tracing::info!(
        "flushed 1 find_email slot (send_headroom={remaining}, in_pipeline={in_pipeline})"
    );
    Ok(1)
}

/// Drain the READY_TO_EMAIL pool into immediate email slots. (`flush_email_queue`)
pub async fn flush_email_queue(db: &SqlitePool, campaign_id: i64) -> Result<i64> {
    if has_pending(db, TaskType::Email, campaign_id).await? {
        return Ok(0);
    }
    let remaining = Mailbox::remaining_today(db).await?;
    if remaining <= 0 {
        return Ok(0);
    }
    let queued = count_scalar(
        db,
        "SELECT COUNT(*) FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
         WHERE d.campaign_id = ?1 AND d.state = 'Ready to Email' AND l.disqualified = 0",
        campaign_id,
    )
    .await?;
    let n = queued.min(remaining);
    if n <= 0 {
        return Ok(0);
    }
    create_lazy_slots(db, TaskType::Email, campaign_id, n).await?;
    tracing::info!("flushed {n} email slots to send now (queued={queued}, cap_remaining={remaining})");
    Ok(n)
}

/// Drain due EMAILED deals into immediate follow-up slots. (`flush_follow_up_queue`)
pub async fn flush_follow_up_queue(db: &SqlitePool, campaign_id: i64) -> Result<i64> {
    if has_pending(db, TaskType::FollowUp, campaign_id).await? {
        return Ok(0);
    }
    let remaining = Mailbox::remaining_today(db).await?;
    if remaining <= 0 {
        return Ok(0);
    }
    let now = Utc::now().to_rfc3339();
    let row = sqlx::query(
        "SELECT COUNT(*) AS n FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
         WHERE d.campaign_id = ?1 AND d.state = 'Emailed' AND d.outcome = ''
           AND l.disqualified = 0 AND d.next_follow_up_at IS NOT NULL AND d.next_follow_up_at <= ?2",
    )
    .bind(campaign_id)
    .bind(now)
    .fetch_one(db)
    .await?;
    let due: i64 = row.get("n");
    let n = due.min(remaining);
    if n <= 0 {
        return Ok(0);
    }
    create_lazy_slots(db, TaskType::FollowUp, campaign_id, n).await?;
    tracing::info!("flushed {n} follow_up slots to run now (due={due}, cap_remaining={remaining})");
    Ok(n)
}

/// Mint the next poll of an in-flight lookup — the collect leg's bound row.
/// (`schedule_collect_email`)
pub async fn schedule_collect_email(db: &SqlitePool, payload: Value, delay_seconds: i64) -> Result<()> {
    let at = Utc::now() + Duration::seconds(delay_seconds);
    Task::create(db, TaskType::CollectEmail, at, &payload).await?;
    Ok(())
}

/// Reset RUNNING tasks to PENDING (crash recovery). (`_recover_stale_running_tasks`)
async fn recover_stale_running_tasks(db: &SqlitePool) -> Result<i64> {
    let res = sqlx::query("UPDATE core_task SET status = 'pending' WHERE status = 'running'")
        .execute(db)
        .await?;
    let n = res.rows_affected() as i64;
    if n > 0 {
        tracing::info!("Recovered {n} stale running tasks");
    }
    Ok(n)
}

/// Recover stale tasks, then top up the drains for every campaign. (`reconcile`)
pub async fn reconcile(db: &SqlitePool, session: &OperatorSession) -> Result<()> {
    recover_stale_running_tasks(db).await?;
    for campaign in &session.campaigns {
        flush_find_email_queue(db, campaign.id).await?;
        flush_email_queue(db, campaign.id).await?;
        flush_follow_up_queue(db, campaign.id).await?;
    }
    let pending = sqlx::query("SELECT COUNT(*) AS n FROM core_task WHERE status = 'pending'")
        .fetch_one(db)
        .await?
        .get::<i64, _>("n");
    tracing::info!("Task queue reconciled: {pending} pending tasks");
    Ok(())
}
\n// revival 2026 touch: src/scheduler.rs\n\n// revival 2026 touch: src/scheduler.rs\n\n// revival 2026 touch: src/scheduler.rs\n
// revival 2026 update: src/scheduler.rs
