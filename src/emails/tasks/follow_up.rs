//! EMAIL follow-up task — the agentic loop for one due EMAILED deal.
//! (`emails/tasks/follow_up.py`)

use anyhow::Result;
use chrono::{Duration, Utc};
use owo_colors::{AnsiColors, OwoColorize};
use sqlx::SqlitePool;

use crate::agents::follow_up::{run_follow_up_agent, FollowUpDecision};
use crate::llm::LlmConfig;
use crate::models::deal::DealState;
use crate::models::{Campaign, ChatMessage, Deal, Lead, Mailbox};
use crate::emails::sender;
use crate::session::OperatorSession;

/// Pick the oldest due EMAILED deal with box headroom, run the agent, execute.
/// (`handle_follow_up`)
pub async fn handle(
    db: &SqlitePool,
    llm: &LlmConfig,
    session: &OperatorSession,
    campaign: &Campaign,
) -> Result<()> {
    let Some(deal) = next_follow_up_deal(db, campaign.id).await? else {
        tracing::info!("follow_up: nothing due (no EMAILED deal past its countdown with box headroom)");
        return Ok(());
    };
    let lead = Lead::get(db, deal.lead_id).await?.expect("deal has a lead");
    let public_id = lead.profile_url.clone();
    tracing::info!("{} {public_id}", "▶ follow_up".color(AnsiColors::Green).bold());

    let decision = run_follow_up_agent(db, llm, session, &deal, campaign).await?;
    match decision.action.as_str() {
        "send_message" => send_reply(db, session, campaign, &deal, &lead, &decision).await?,
        "mark_completed" => complete(db, campaign, &public_id, &decision).await?,
        "wait" => rearm(db, deal.id, &decision).await?,
        other => tracing::warn!("follow_up: unknown action {other}"),
    }
    Ok(())
}

/// Oldest due EMAILED deal whose bound box still has headroom. (`_next_follow_up_deal`)
async fn next_follow_up_deal(db: &SqlitePool, campaign_id: i64) -> Result<Option<Deal>> {
    for deal in Deal::get_followup_due(db, campaign_id).await? {
        // The get_followup_due query already filters outcome=''; re-check the box cap.
        if let Some(mailbox_id) = deal.mailbox_id {
            if let Some(box_) = Mailbox::get(db, mailbox_id).await? {
                let sent = box_.sent_today(db).await?;
                if box_.headroom_today(sent) > 0 {
                    return Ok(Some(deal));
                }
            }
        }
    }
    Ok(None)
}

async fn send_reply(
    db: &SqlitePool,
    session: &OperatorSession,
    _campaign: &Campaign,
    deal: &Deal,
    lead: &Lead,
    decision: &FollowUpDecision,
) -> Result<()> {
    let subject = reply_subject(&deal.email_subject);
    let message = decision.message.clone().unwrap_or_default();
    let mailbox = Mailbox::get(db, deal.mailbox_id.unwrap_or(0))
        .await?
        .ok_or_else(|| anyhow::anyhow!("follow_up: deal has no bound mailbox"))?;
    let to = lead.email.clone().unwrap_or_default();
    let in_reply_to = ChatMessage::latest_external_id(db, deal.id)
        .await?
        .unwrap_or_else(|| deal.email_message_id.clone());
    let bcc = if session.user.email.is_empty() { None } else { Some(session.user.email.as_str()) };

    tracing::info!("follow_up reply to {}: {message}", lead.profile_url);
    let message_id = sender::send_email(
        &mailbox,
        &to,
        &subject,
        &message,
        bcc,
        Some(&in_reply_to),
        Some(&deal.email_message_id),
    )
    .await?;
    ChatMessage::create(db, deal.id, &message, &message_id, true, Some(session.user.id)).await?;
    let next = Utc::now() + hours(decision.follow_up_hours);
    Deal::set_next_follow_up(db, deal.id, next).await?;
    Ok(())
}

async fn complete(
    db: &SqlitePool,
    campaign: &Campaign,
    public_id: &str,
    decision: &FollowUpDecision,
) -> Result<()> {
    let outcome = decision.outcome.clone().unwrap_or_default();
    Deal::set_profile_state(db, campaign.id, public_id, DealState::Completed, "", &outcome).await?;
    tracing::info!("follow_up completed for {public_id}: outcome={outcome}");
    Ok(())
}

async fn rearm(db: &SqlitePool, deal_id: i64, decision: &FollowUpDecision) -> Result<()> {
    let next = Utc::now() + hours(decision.follow_up_hours);
    Deal::set_next_follow_up(db, deal_id, next).await?;
    Ok(())
}

/// `Re:` the opener's subject, without stacking a second `Re:`. (`_reply_subject`)
fn reply_subject(opener_subject: &str) -> String {
    if opener_subject.to_lowercase().starts_with("re:") {
        opener_subject.to_string()
    } else {
        format!("Re: {opener_subject}")
    }
}

fn hours(h: f64) -> Duration {
    Duration::milliseconds((h * 3_600_000.0) as i64)
}
