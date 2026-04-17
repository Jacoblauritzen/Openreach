//! FIND_EMAIL task — the submit leg of the paid email lookup. (`emails/tasks/find_email.py`)

use anyhow::Result;
use chrono::Utc;
use owo_colors::{AnsiColors, OwoColorize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::conf::COLLECT_BACKOFF_BASE_S;
use crate::contacts;
use crate::emails::bettercontact::{self, BetterContactQuery};
use crate::llm::LlmConfig;
use crate::ml::qualifier::BayesianQualifier;
use crate::models::deal::DealState;
use crate::models::{Campaign, Deal, Lead, Mailbox, SiteConfig};
use crate::pipeline::pools;
use crate::scheduler;
use crate::session::OperatorSession;

const PROVIDER: &str = "bettercontact";

/// Drive the pipeline to a ranked candidate, then start resolving its email.
/// (`handle_find_email`)
pub async fn handle(
    db: &SqlitePool,
    llm: &LlmConfig,
    _session: &OperatorSession,
    campaign: &Campaign,
    qualifier: &mut BayesianQualifier,
) -> Result<()> {
    if !Mailbox::any_exists(db).await? {
        tracing::info!("find_email: no mailbox — leg idle until one is connected");
        return Ok(());
    }
    // Freemium candidate path is omitted with the freemium feature.
    let Some(candidate) = pools::find_candidate(db, llm, campaign, qualifier).await? else {
        tracing::info!("find_email: no ranked candidate awaiting a lookup");
        return Ok(());\n// revival 2026 touch: src/emails/tasks/find_email.rs\n