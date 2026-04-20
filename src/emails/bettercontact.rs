//! BetterContact email lookup — a port of `openhaze/emails/bettercontact.py`.
//!
//! The paid finder is a **two-leg async handshake** so the daemon never blocks on
//! a poll: [`submit`] fires one job and returns its `request_id`, and [`poll_once`]
//! checks that job exactly once (no wait), reporting `running` / `hit` / `miss`.
//! The collect task owns the retry backoff between polls. A missing key or an
//! unreachable service yields [`BetterContactError::Unavailable`] (never a bare
//! error) so enrichment can't take down the daemon.
//!
//! The blocking [`submit_and_poll`] transport is shared with Lead Finder
//! discovery (`discovery::search`), which legitimately waits inside its handler.
//! Here "blocking" is an `await` — same contract, no thread is parked.

use std::time::Duration;

use serde_json::{json, Value};

const ENRICH_URL: &str = "https://app.bettercontact.rocks/api/v2/async";
const POLL_INTERVAL: Duration = Duration::from_secs(5);
const POLL_TIMEOUT: Duration = Duration::from_secs(300);
const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

/// Email statuses BetterContact considers usable. (`_USABLE_STATUSES`)
const USABLE_STATUSES: [&str; 3] = ["valid", "deliverable", "catch_all_safe"];

/// Cloudflare 403s a non-browser User-Agent (error 1010), so spoof a browser.
const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

/// BetterContact could not run — no key configured, or the service was
/// unreachable. Distinct from a genuine miss (it ran, found no email).
/// (`BetterContactUnavailable`)
#[derive(Debug, thiserror::Error)]
pub enum BetterContactError {
    #[error("BetterContact unavailable: {0}")]
    Unavailable(String),
}

type Result<T> = std::result::Result<T, BetterContactError>;

/// A lead to resolve. `linkedin_url` alone works; name/company lift the hit rate.
/// (`BetterContactQuery`)
#[derive(Debug, Clone, Default)]
pub struct BetterContactQuery {
    pub linkedin_url: String,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub company_domain: String,
}

/// (`BetterContactResult`)
#[derive(Debug, Clone)]
pub struct BetterContactResult {
    pub email: String,
    pub status: String,
}

/// Result of a single poll of an in-flight lookup. (`PollOutcome`)
#[derive(Debug, Clone)]
pub struct PollOutcome {
    pub running: bool,
    pub email: String,
}

impl PollOutcome {
    pub fn running() -> Self {
        PollOutcome { running: true, email: String::new() }
    }
    pub fn terminated(email: String) -> Self {
        PollOutcome { running: false, email }
    }
    /// Terminated with a usable email.
    pub fn is_hit(&self) -> bool {
        !self.running && !self.email.is_empty()
    }
    /// Terminated with no usable email — a genuine, terminal miss.
    pub fn is_miss(&self) -> bool {
        !self.running && self.email.is_empty()
    }
}

/// True when a non-empty API key is set.
pub fn is_configured(api_key: &str) -> bool {
    !api_key.trim().is_empty()
}

fn require_key(api_key: &str) -> Result<&str> {
    let key = api_key.trim();
    if key.is_empty() {
        return Err(BetterContactError::Unavailable(
            "no BetterContact API key configured".into(),
        ));
    }
    Ok(key)
}

fn client(api_key: &str) -> Result<reqwest::Client> {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
    let mut headers = HeaderMap::new();