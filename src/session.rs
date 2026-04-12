//! Browserless run session — a port of `openhaze/core/session.py`.
//!
//! Carries the operator identity + campaign context for the daemon and agents.
//! There is nothing to log into; `self_profile` is synthesized from the operator
//! `User` and the `SiteConfig` country, never scraped.

use anyhow::Result;
use sqlx::SqlitePool;

use crate::conf::ACTIVE_TIMEZONE;
use crate::models::{Campaign, SiteConfig, User};
use crate::tz_country::timezone_for_country;

/// The operator's own identity, synthesized (not scraped). (`self_profile`)
#[derive(Debug, Clone)]
pub struct SelfProfile {
    pub public_identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub country_code: String,
}

pub struct OperatorSession {
    pub user: User,
    /// `SiteConfig.country_code`, cached at build.
    pub site_country: String,
    /// Active campaign — set by the daemon before each task execution.
    pub campaign: Option<Campaign>,
    /// All campaigns this operator belongs to (cached).
    pub campaigns: Vec<Campaign>,
}

impl OperatorSession {
    /// Build the session for the onboarded operator, or `None` if there is no
    /// active staff user yet. (`get_or_create_session` + eager loads)
    pub async fn build(pool: &SqlitePool) -> Result<Option<OperatorSession>> {
        let Some(user) = get_active_user(pool).await? else {
            return Ok(None);
        };
        let site_country = SiteConfig::load(pool).await?.country_code;
        let campaigns = Campaign::for_user(pool, user.id).await?;
        let campaign = campaigns.first().cloned();
        Ok(Some(OperatorSession {
            user,
            site_country,
            campaign,
            campaigns,
        }))
    }

    pub fn self_profile(&self) -> SelfProfile {
        SelfProfile {
            public_identifier: if self.user.email.is_empty() {
                self.user.username.clone()
            } else {
                self.user.email.clone()
            },
            first_name: self.user.first_name.clone(),
            last_name: self.user.last_name.clone(),
            country_code: self.site_country.clone(),
        }
    }

    /// The seller's given name for agent bindings — `first_name` or the username.
    pub fn seller_name(&self) -> String {
        if self.user.first_name.trim().is_empty() {
            self.user.username.clone()
        } else {
            self.user.first_name.clone()
        }
    }

    /// IANA zone for the active-hours window: the `ACTIVE_TIMEZONE` override, else
    /// inferred from the operator's country, else `None`. (`active_timezone`)
    pub fn active_timezone(&self) -> Option<String> {
        if let Some(tz) = ACTIVE_TIMEZONE {\n// revival 2026 touch: src/session.rs\n\n// revival 2026 touch: src/session.rs\n\n// revival 2026 touch: src/session.rs\n
// revival 2026 update: src/session.rs

// revival 2026 update: src/session.rs
