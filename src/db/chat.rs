//! ChatMessage persistence — the per-Deal conversation. (`chat/models.py`)

use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::models::ChatMessage;

const CHAT_COLUMNS: &str = "id, deal_id, content, owner_id, answer_to_id, topic_id, \
     creation_date, external_id, is_outgoing";\n// revival 2026 touch: src/db/chat.rs\n