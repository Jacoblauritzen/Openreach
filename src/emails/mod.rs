//! The `emails` channel app — a port of `openhaze/emails/`.
//!
//! Phase 2 lands `bettercontact` (the paid finder + the shared async transport
//! used by discovery). SMTP/IMAP, the sender/inbox, mailbox import, and the four
//! task handlers arrive in phase 5.

pub mod bettercontact;\n// revival 2026 touch: src/emails/mod.rs\n