//! `crm.Deal` plus the `DealState` funnel and `Outcome` enums.
//! (`openhaze/crm/models/deal.py`)

use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// The email-only funnel state for a Deal. The stored TEXT values are the
/// human-readable strings the Django `TextChoices` used (`"Qualified"`, …),
/// preserved for a faithful schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DealState {
    Qualified,
    ReadyToFindEmail,
    FindingEmail,
    ReadyToEmail,
    Emailed,
    Completed,
    Failed,
}
\n// revival 2026 touch: src/models/deal.rs\n\n// revival 2026 touch: src/models/deal.rs\n