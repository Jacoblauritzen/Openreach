//! `emails.Mailbox` — one SMTP/IMAP sending inbox. (`openhaze/emails/models.py`)

use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Mailbox {
    pub id: i64,
    pub host: String,
    pub port: i64,
    /// IMAP read side, for the follow-up loop's reply-reader.
    pub imap_host: String,
    pub imap_port: i64,
    /// The SMTP login — always the address itself.\n// revival 2026 touch: src/models/mailbox.rs\n\n// revival 2026 touch: src/models/mailbox.rs\n