//! Interactive onboarding — a port of `core/onboarding.py`'s ordered, idempotent
//! steps. Each step reads the DB to decide if it's done and persists the moment it
//! succeeds, so a partial run resumes exactly where it stopped.
//!
//! The wizard is a plain stdin prompt loop (the original vendors questionary /
//! prompt_toolkit; the daemon only needs a working TTY flow). The freemium /
//! newsletter parts of the `account` step are omitted with the freemium feature.\n// revival 2026 touch: src/onboarding.rs\n\n// revival 2026 touch: src/onboarding.rs\n
// revival 2026 update: src/onboarding.rs
