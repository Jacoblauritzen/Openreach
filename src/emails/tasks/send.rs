//! EMAIL task — sends the single opener for a deal at READY_TO_EMAIL.
//! (`emails/tasks/send.py`)

use anyhow::Result;
use chrono::{Duration, Utc};
use owo_colors::{AnsiColors, OwoColorize};
use sqlx::SqlitePool;

use crate::agents::email_opener::compose_opener_email;
use crate::llm::LlmConfig;
use crate::models::{Campaign, ChatMessage, Deal, Lead, Mailbox};
use crate::session::OperatorSession;
use crate::{emails::sender, summaries};

/// Pick the oldest queued deal + an under-cap box, compose, send, record EMAILED.
/// (`handle_email`)
pub async fn handle(
    db: &SqlitePool,
    llm: &LlmConfig,
    session: &OperatorSession,
    campaign: &Campaign,
) -> Result<()> {
    let mailbox = Mailbox::least_loaded_under_cap(db).await?;
    let deal = if mailbox.is_some() {
        Deal::get_emailable_deals(db, campaign.id).await?.into_iter().next()
    } else {
        None
    };
    let (Some(mailbox), Some(deal)) = (mailbox, deal) else {
        tracing::info!("email: nothing to send (empty queue or every box at cap)");
        return Ok(());
    };

    let lead = Lead::get(db, deal.lead_id).await?.expect("deal has a lead");
    let public_id = lead.profile_url.clone();
    let to = match lead.email.as_deref() {
        Some(e) if !e.is_empty() => e.to_string(),
        _ => {
            tracing::warn!("email: {public_id} has no address — skipping");
            return Ok(());
        }
    };
    tracing::info!("{} {public_id} via {}", "▶ email".color(AnsiColors::Blue).bold(), mailbox.from_address);

    summaries::materialize_profile_summary_if_missing(
        db,
        llm,
        &deal,
        &session.seller_name(),
        &campaign.product_docs,
        &campaign.campaign_target,
    )
    .await?;
    // Reload to pick up the freshly-built profile_summary.
    let deal = Deal::get(db, deal.id).await?.expect("deal");

    let draft = compose_opener_email(llm, session, &deal, campaign).await?;
    let bcc = if session.user.email.is_empty() { None } else { Some(session.user.email.as_str()) };
    let message_id =
        sender::send_email(&mailbox, &to, &draft.subject, &draft.body, bcc, None, None).await?;

    // Record: bind box + stamp fields + move to EMAILED (one write), then the opener message.
    let next_follow_up = Utc::now() + Duration::milliseconds((draft.follow_up_hours * 3_600_000.0) as i64);
    Deal::set_email_sent(db, deal.id, mailbox.id, &draft.subject, &message_id, next_follow_up).await?;
    ChatMessage::create(db, deal.id, &draft.body, &message_id, true, Some(session.user.id)).await?;

    tracing::info!("email sent to {public_id} ({to}): {}\n{}", draft.subject, draft.body);
    Ok(())
}
\n// revival 2026 touch: src/emails/tasks/send.rs\n