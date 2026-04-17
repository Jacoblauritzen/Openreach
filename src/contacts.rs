//! The central contacts store (the hub) — ask before paying BetterContact, give
//! back what we find. (`contacts/service.py`)
//!
//! Two best-effort calls; a missing token or an outage degrades to a no-op and
//! never breaks outreach. The EEA/UK/CH geo-gate is enforced server-side; the
//! cheap client check just avoids a pointless round-trip.

use anyhow::Result;
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::geo::is_eea_located;
use crate::models::{Lead, SiteConfig};
use crate::session::OperatorSession;

const DEFAULT_API_URL: &str = "https://hub.openhaze.app";
const TIMEOUT_S: u64 = 30;

/// Where a contributed address came from.
pub const ORIGIN_BETTERCONTACT: &str = "bettercontact";
pub const ORIGIN_PROFILE_INFO: &str = "profile_info";

fn http() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(TIMEOUT_S))
        .build()
        .unwrap_or_default()
}

fn endpoint(cfg: &SiteConfig, path: &str) -> String {
    let base = if cfg.contacts_api_url.is_empty() {
        DEFAULT_API_URL
    } else {
        cfg.contacts_api_url.trim_end_matches('/')
    };
    format!("{base}/api/v2/{path}/")
}

/// A stored email for `lead`, or `None` (miss, no token, or outage). (`resolve`)
pub async fn resolve(db: &SqlitePool, lead: &Lead) -> Result<Option<String>> {
    let cfg = SiteConfig::load(db).await?;
    if cfg.contacts_api_token.is_empty() {
        return Ok(None);
    }
    let resp = http()
        .get(endpoint(&cfg, "resolve"))
        .query(&[("id", &lead.profile_url)])
        .bearer_auth(&cfg.contacts_api_token)
        .send()
        .await;
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            tracing::info!("hub: resolve unavailable for {}: {e}", lead.profile_url);
            return Ok(None);
        }
    };
    if !matches!(resp.status().as_u16(), 200 | 404) {
        return Ok(None);
    }
    let body: Value = resp.json().await.unwrap_or(Value::Null);
    let email = body
        .get("emails")
        .and_then(|e| e.as_array())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .map(str::to_string);
    if let Some(ref e) = email {
        tracing::info!("hub: resolved {e} for {} (saved a paid lookup)", lead.profile_url);
    }
    Ok(email)
}

/// Give `lead`'s email(s) to the store — best-effort, non-EEA only. (`contribute`)
pub async fn contribute(
    db: &SqlitePool,
    session: &OperatorSession,
    lead: &Lead,
    emails: &[String],
    origin: &str,
) -> Result<()> {
    let cfg = SiteConfig::load(db).await?;
    if is_eea_located(Some(&cfg.country_code)) {
        tracing::debug!("hub: operator in EEA/UK/CH — skipping give-back for {}", lead.profile_url);
        return Ok(());
    }
    let emails: Vec<&String> = emails.iter().filter(|e| !e.is_empty()).collect();
    if emails.is_empty() {
        return Ok(());
    }
    if is_eea_located(Some(&lead.country_code)) {
        tracing::debug!("hub: skipping {} — EEA/UK/CH lead, out of store scope", lead.profile_url);