//! Email opener agent — composes the single cold email for a deal.
//! (`core/agents/email_opener.py`)

use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

use crate::agents::prompt::base_context;
use crate::llm::{self, LlmConfig};
use crate::models::{Campaign, Deal};
use crate::prompts;
use crate::session::OperatorSession;

/// Structured output from the email opener agent. (`EmailDraft`)
#[derive(Debug, Clone, Deserialize)]
pub struct EmailDraft {
    pub subject: String,
    pub body: String,
    pub follow_up_hours: f64,
}

fn schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "subject": {"type": "string", "description": "Short, specific subject like a real person wrote — not salesy."},
            "body": {"type": "string", "description": "A few short sentences; no signature, no placeholders."},
            "follow_up_hours": {"type": "number", "description": "Hours to wait before first follow-up (48-72h reasonable cold-open default)."}