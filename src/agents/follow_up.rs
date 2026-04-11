//! Follow-up agent — reads the email thread, returns a structured decision.
//! (`core/agents/follow_up.py`)

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::agents::prompt::{base_context, format_facts};
use crate::emails::inbox;
use crate::llm::{self, LlmConfig};
use crate::models::{Campaign, ChatMessage, Deal};
use crate::prompts;
use crate::session::OperatorSession;

/// Trailing verbatim messages the agent sees alongside the rolling summary.
/// (`RECENT_MESSAGES_WINDOW`)
const RECENT_MESSAGES_WINDOW: usize = 6;

/// Structured output from the follow-up agent. (`FollowUpDecision`)
#[derive(Debug, Clone, Deserialize)]
pub struct FollowUpDecision {
    pub action: String, // send_message | mark_completed | wait
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub outcome: Option<String>,
    pub follow_up_hours: f64,
}

impl FollowUpDecision {
    /// Validate the required-field invariants. (`_check_required_fields`)
    pub fn validate(&self) -> Result<()> {
        if self.action == "send_message" && self.message.as_deref().unwrap_or("").is_empty() {
            return Err(anyhow!("message is required when action='send_message'"));
        }
        if self.action == "mark_completed" && self.outcome.as_deref().unwrap_or("").is_empty() {
            return Err(anyhow!("outcome is required when action='mark_completed'"));
        }
        Ok(())
    }
}

fn schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"type": "string", "enum": ["send_message", "mark_completed", "wait"]},
            "message": {"type": ["string", "null"], "description": "Required when action='send_message'."},
            "outcome": {"type": ["string", "null"],
                "enum": ["converted", "not_interested", "wrong_fit", "no_budget", "has_solution", "bad_timing", "unresponsive", null],
                "description": "Required when action='mark_completed'."},
            "follow_up_hours": {"type": "number"}
        },
        "required": ["action", "follow_up_hours"]
    })
}

fn humanize_age(when: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let delta = now - when;
    if delta < chrono::Duration::hours(1) {
        format!("{}m ago", (delta.num_minutes()).max(1))
    } else if delta < chrono::Duration::days(1) {
        format!("{}h ago", delta.num_hours())
    } else {
        format!("{}d ago", delta.num_days())
    }\n// revival 2026 touch: src/agents/follow_up.rs\n