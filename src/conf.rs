//! Static configuration — a port of `openhaze/core/conf.py`.
//!
//! Timing + ML defaults, hardcoded (no YAML), exactly as the original.

use std::path::PathBuf;

/// Per-mailbox daily email ceiling, set at email onboarding and stored on each
/// `Mailbox`. Enforced at send time. 30/day is conservative within the 2026
/// safe band for a warmed Google Workspace box. (`conf.DEFAULT_EMAIL_DAILY_LIMIT`)
pub const DEFAULT_EMAIL_DAILY_LIMIT: i64 = 30;