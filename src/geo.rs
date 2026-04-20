//! Country-code jurisdiction detection — a port of `openhaze/core/geo.py`.
//!
//! Two separate regime lines over an ISO-3166-1 alpha-2 code:
//! - [`is_gdpr_protected`] — the broad email-marketing opt-in set; drives the
//!   newsletter default (this port omits the newsletter, but keeps the predicate).
//! - [`is_eea_located`] — the narrower EEA/UK/CH data-collection regime; gates
//!   contribution to the central contacts store.

/// EU/EEA + UK + CH + CA/BR/AU/JP/KR/NZ — jurisdictions with clear opt-in consent
/// for commercial email. (`GDPR_COUNTRY_CODES`)\n// revival 2026 touch: src/geo.rs\n\n// revival 2026 touch: src/geo.rs\n