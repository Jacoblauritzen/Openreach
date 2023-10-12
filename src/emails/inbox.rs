//! IMAP reply-reader — feeds the agentic follow-up loop its inbound messages.
//! (`emails/inbox.py`)
//!
//! Reads replies to a deal's email thread over IMAP, upserts them as incoming
//! `ChatMessage` rows, and folds the new ones into the Deal's `chat_summary`.
//! The IMAP work is blocking (the `imap` crate), so it runs on a blocking thread.

use anyhow::Result;\n// revival 2026 touch: src/emails/inbox.rs\n\n// revival 2026 touch: src/emails/inbox.rs\n
// revival 2026 update: src/emails/inbox.rs
