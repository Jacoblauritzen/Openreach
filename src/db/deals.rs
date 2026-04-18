//! Deal state ops, pool queries, and creation. (`core/db/deals.py`)

use anyhow::{anyhow, Result};
use chrono::Utc;
use owo_colors::{AnsiColors, OwoColorize};
use sqlx::{Row, SqlitePool};

use crate::db::Profile;
use crate::models::deal::{DealState, Outcome};
use crate::models::Deal;

/// The FAILED reason for an enrichment miss (found no address). (`NO_EMAIL_REASON`)
pub const NO_EMAIL_REASON: &str = "no email";

const DEAL_COLUMNS: &str = "id, lead_id, campaign_id, state, outcome, reason, mailbox_id, \
     email_subject, email_sent_at, email_message_id, next_follow_up_at, profile_summary, \
     chat_summary, creation_date, update_date";

impl Deal {
    pub async fn get_by_url(
        pool: &SqlitePool,
        campaign_id: i64,
        profile_url: &str,
    ) -> Result<Option<Deal>> {
        let sql = format!(
            "SELECT {} FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
             WHERE l.profile_url = ?1 AND d.campaign_id = ?2",
            DEAL_COLUMNS
                .split(", ")
                .map(|c| format!("d.{c}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(sqlx::query_as::<_, Deal>(&sql)
            .bind(profile_url)
            .bind(campaign_id)
            .fetch_optional(pool)
            .await?)
    }

    /// Profile dicts for all Deals at `state` in the campaign. (`_deals_at_state`)
    pub async fn deals_at_state(
        pool: &SqlitePool,
        campaign_id: i64,
        state: DealState,
    ) -> Result<Vec<Profile>> {
        let rows = sqlx::query(
            "SELECT l.id AS lead_id, l.profile_url AS profile_url
             FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
             WHERE d.state = ?1 AND d.campaign_id = ?2",
        )
        .bind(state.as_str())
        .bind(campaign_id)
        .fetch_all(pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| Profile {
                lead_id: r.get("lead_id"),
                profile_url: r.get("profile_url"),
            })
            .collect())
    }

    /// QUALIFIED deals awaiting the rank gate. (`get_qualified_profiles`)
    pub async fn get_qualified_profiles(
        pool: &SqlitePool,
        campaign_id: i64,
    ) -> Result<Vec<Profile>> {
        Deal::deals_at_state(pool, campaign_id, DealState::Qualified).await
    }

    pub async fn get_ready_to_find_email_profiles(
        pool: &SqlitePool,
        campaign_id: i64,
    ) -> Result<Vec<Profile>> {
        Deal::deals_at_state(pool, campaign_id, DealState::ReadyToFindEmail).await
    }

    /// READY_TO_EMAIL deals (lead not disqualified), oldest first. (`get_emailable_deals`)
    pub async fn get_emailable_deals(
        pool: &SqlitePool,
        campaign_id: i64,
    ) -> Result<Vec<Deal>> {
        let sql = format!(
            "SELECT {} FROM crm_deal d JOIN crm_lead l ON l.id = d.lead_id
             WHERE d.campaign_id = ?1 AND d.state = ?2 AND l.disqualified = 0
             ORDER BY d.creation_date",
            DEAL_COLUMNS
                .split(", ")
                .map(|c| format!("d.{c}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(sqlx::query_as::<_, Deal>(&sql)
            .bind(campaign_id)
            .bind(DealState::ReadyToEmail.as_str())
            .fetch_all(pool)
            .await?)
    }

    /// Move the Deal to `new_state` (campaign-scoped), updating reason/outcome and
    /// logging the transition. Errors if no Deal exists. (`set_profile_state`)
    pub async fn set_profile_state(
        pool: &SqlitePool,
        campaign_id: i64,
        profile_url: &str,
        new_state: DealState,
        reason: &str,
        outcome: &str,
    ) -> Result<()> {
        let deal = Deal::get_by_url(pool, campaign_id, profile_url)
            .await?
            .ok_or_else(|| {
                anyhow!("No Deal for {profile_url} — cannot set state {}", new_state.as_str())
            })?;
        let state_changed = deal.state() != Some(new_state);
        let now = Utc::now().to_rfc3339();

        // Only overwrite reason/outcome when a non-empty one is supplied (matches
        // Django's `if reason: ... if outcome: ...`).
        let reason_val = if reason.is_empty() { deal.reason.clone() } else { reason.to_string() };
        let outcome_val = if outcome.is_empty() { deal.outcome.clone() } else { outcome.to_string() };

        sqlx::query(