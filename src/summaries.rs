//! mem0-style fact-list summaries for Deal profile + chat history.
//! (`core/db/summaries.py`)
//!
//! Summaries are stored as JSON `{"facts": [...]}` on `Deal.profile_summary` and
//! `Deal.chat_summary`. Both are campaign-scoped derived caches rebuilt from source
//! (the lead's `profile_text`, and `ChatMessage` rows). The mem0 UPDATE prompt is
//! reimplemented functionally (ADD/UPDATE/DELETE/NONE reconciliation) rather than
//! vendoring `mem0ai`.

use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::llm::{self, LlmConfig};
use crate::models::{ChatMessage, Deal, Lead};

/// Fact-extraction system prompt — modeled on mem0's FACT_RETRIEVAL_PROMPT.
const FACT_EXTRACTION_PROMPT: &str = "\
You are an information-extraction assistant. Read the input text and produce a flat \
list of atomic, self-contained factual statements about the lead (the person we are \
talking to).\n\
Rules:\n\
- Each fact must be a complete sentence that stands on its own.\n\
- Prefer concrete, durable facts (identity, role, employer, location, career arc, \
stated goals, expressed concerns) over fleeting commentary.\n\
- Do not invent facts. If the text does not assert it, do not include it.\n\
- Do not duplicate facts. Merge near-duplicates.\n\
- Keep each fact short (under ~25 words).\n\
- Return between 0 and 30 facts. Empty list is acceptable.\n\
- The input may contain messages tagged [Me] and [Lead]. Extract facts about the \
LEAD only; use [Me] messages solely as context to disambiguate the lead's replies.";

#[derive(Debug, Deserialize)]
struct FactList {
    #[serde(default)]
    facts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MemoryAction {
    #[serde(default)]
    id: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    event: String,
}

#[derive(Debug, Deserialize)]
struct ReconcileResponse {
    #[serde(default)]
    memory: Vec<MemoryAction>,
}

fn identity_binding(seller_name: &str) -> String {
    format!(
        "\nIdentity binding (read carefully):\n- [Me] is named {seller_name}.\n\
         - When a [Lead] message mentions `{seller_name}`, that is a reference to [Me] — \
         never attribute it as a fact about the lead."
    )
}

/// Extract a flat list of atomic facts from `text`. (`extract_facts`)
pub async fn extract_facts(
    llm: &LlmConfig,
    text: &str,
    seller_name: &str,
    context: &str,
) -> Result<Vec<String>> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut preamble = format!("{FACT_EXTRACTION_PROMPT}{}", identity_binding(seller_name));
    if !context.is_empty() {
        preamble.push_str(&format!("\n\nContext for relevance:\n{context}"));
    }
    let user = format!("{preamble}\n\nInput text:\n{text}");
    let schema = json!({
        "type": "object",
        "properties": {"facts": {"type": "array", "items": {"type": "string"}}},
        "required": ["facts"]
    });
    let result: FactList = llm::complete_structured(llm, &user, &schema, 0.0).await?;
    Ok(result.facts)
}

/// Reconcile `new_facts` against `existing` via a mem0-style UPDATE step.
/// (`reconcile_facts`)
pub async fn reconcile_facts(
    llm: &LlmConfig,
    existing: &[String],
    new_facts: &[String],
    seller_name: &str,
) -> Result<Vec<String>> {
    if new_facts.is_empty() {
        return Ok(existing.to_vec());
    }
    let old_listing = existing
        .iter()
        .enumerate()
        .map(|(i, f)| format!("  {{\"id\": \"{i}\", \"text\": {:?}}}", f))
        .collect::<Vec<_>>()
        .join(",\n");
    let new_listing = new_facts
        .iter()
        .map(|f| format!("- {f}"))
        .collect::<Vec<_>>()
        .join("\n");
    let prompt = format!(
        "You maintain a memory of facts about a lead. Reconcile the NEW facts into the \
         EXISTING memory. For each change, emit one action with an `event`:\n\
         - ADD: a genuinely new fact (id is a fresh string).\n\
         - UPDATE: an existing fact should be revised (reuse its id, new `text`).\n\
         - DELETE: an existing fact is wrong or contradicted (reuse its id).\n\
         - NONE: no change.\n\
         Context: in the source conversation, [Me] is {seller_name}. Existing facts that \
         describe `{seller_name}` as if they were the lead are contamination — DELETE them.\n\n\
         EXISTING memory:\n[\n{old_listing}\n]\n\nNEW facts:\n{new_listing}\n\n\
         Return a JSON object {{\"memory\": [{{\"id\", \"text\", \"event\"}}...]}}.",
    );
    let schema = json!({
        "type": "object",
        "properties": {"memory": {"type": "array", "items": {"type": "object",
            "properties": {"id": {"type": "string"}, "text": {"type": "string"},
                "event": {"type": "string", "enum": ["ADD", "UPDATE", "DELETE", "NONE"]}}}}}
    });
    let resp: ReconcileResponse = llm::complete_structured(llm, &prompt, &schema, 0.0).await?;
    Ok(apply_memory_actions(existing, &resp.memory))
}

/// Apply ADD/UPDATE/DELETE/NONE to a flat fact list keyed by index.
/// (`_apply_memory_actions`)
fn apply_memory_actions(existing: &[String], actions: &[MemoryAction]) -> Vec<String> {
    // Ordered store of (id, text).
    let mut store: Vec<(String, String)> = existing
        .iter()
        .enumerate()
        .map(|(i, f)| (i.to_string(), f.clone()))
        .collect();
    let mut next_id = existing.len();

    for action in actions {
        if action.text.is_empty() && action.event != "DELETE" {
            continue;
        }
        match action.event.as_str() {
            "ADD" => {
                store.push((next_id.to_string(), action.text.clone()));
                next_id += 1;
            }
            "UPDATE" => {
                if let Some(slot) = store.iter_mut().find(|(id, _)| id == &action.id) {
                    slot.1 = action.text.clone();
                } else {
                    tracing::warn!("UPDATE skipped: unknown id {:?}", action.id);
                }
            }
            "DELETE" => {
                let before = store.len();
                store.retain(|(id, _)| id != &action.id);
                if store.len() == before {
                    tracing::warn!("DELETE skipped: unknown id {:?}", action.id);
                }
            }
            _ => {} // NONE
        }
    }
    store.into_iter().map(|(_, t)| t).collect()
}

/// Build `deal.profile_summary` lazily on first follow-up touch. (`materialize_profile_summary_if_missing`)
pub async fn materialize_profile_summary_if_missing(
    db: &SqlitePool,
    llm: &LlmConfig,
    deal: &Deal,
    seller_name: &str,
    product_docs: &str,
    campaign_target: &str,
) -> Result<()> {
    if deal.profile_summary.is_some() {
        return Ok(());
    }
    let Some(lead) = Lead::get(db, deal.lead_id).await? else {
        return Ok(());
    };
    if lead.profile_text.is_empty() {
        tracing::warn!(
            "materialize_profile_summary: no profile_text for deal={} lead={}",
            deal.id,
            lead.profile_url
        );
        return Ok(());
    }
    let mut context_parts = Vec::new();
    if !campaign_target.is_empty() {
        context_parts.push(format!("Campaign target: {campaign_target}"));
    }
    if !product_docs.is_empty() {
        context_parts.push(format!("Product context: {product_docs}"));
    }
    let facts = extract_facts(llm, &lead.profile_text, seller_name, &context_parts.join("\n\n")).await?;
    Deal::set_profile_summary(db, deal.id, &json!({"facts": facts}).to_string()).await?;
    tracing::info!(
        "profile_summary built for deal={} lead={} ({} facts)",
        deal.id,
        lead.profile_url,
        facts.len()
    );
    Ok(())
}

/// Fold newly-synced ChatMessages into `deal.chat_summary`. (`update_chat_summary`)
pub async fn update_chat_summary(
    db: &SqlitePool,
    llm: &LlmConfig,
    deal: &Deal,
    new_messages: &[ChatMessage],
    seller_name: &str,
) -> Result<()> {
    let formatted = format_messages_for_extraction(new_messages);
    if formatted.is_empty() {
        return Ok(());
    }
    let new_facts = extract_facts(llm, &formatted, seller_name, "").await?;
    if new_facts.is_empty() {
        return Ok(());
    }
    let existing = deal
        .chat_summary
        .as_deref()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
        .and_then(|v| v.get("facts").and_then(|f| f.as_array()).cloned())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect::<Vec<_>>())
        .unwrap_or_default();
    let reconciled = reconcile_facts(llm, &existing, &new_facts, seller_name).await?;
    Deal::set_chat_summary(db, deal.id, &json!({"facts": reconciled}).to_string()).await?;
    tracing::info!(
        "chat_summary updated for deal={} (+{} new facts → {} total)",
        deal.id,
        new_facts.len(),
        reconciled.len()
    );
    Ok(())
}

/// Render ChatMessages as a labeled transcript; empty if no incoming (lead)
/// messages. (`_format_messages_for_extraction`)
fn format_messages_for_extraction(messages: &[ChatMessage]) -> String {
    let mut lines = Vec::new();
    let mut has_incoming = false;
    for m in messages {
        let content = m.content.trim();
        if content.is_empty() {
            continue;
        }
        let tag = if m.is_outgoing { "[Me]" } else { "[Lead]" };
        if !m.is_outgoing {
            has_incoming = true;
        }
        lines.push(format!("{tag} {content}"));
    }
    if !has_incoming {
        return String::new();
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_actions_add_update_delete() {
        let existing = vec!["a".to_string(), "b".to_string()];
        let actions = vec![
            MemoryAction { id: "0".into(), text: "a2".into(), event: "UPDATE".into() },
            MemoryAction { id: "1".into(), text: String::new(), event: "DELETE".into() },
            MemoryAction { id: "x".into(), text: "c".into(), event: "ADD".into() },
        ];
        let out = apply_memory_actions(&existing, &actions);
        assert_eq!(out, vec!["a2".to_string(), "c".to_string()]);
    }

    #[test]
    fn format_needs_incoming() {
        let msgs = vec![ChatMessage {
            id: 1,
            deal_id: 1,
            content: "hi".into(),
            owner_id: None,
            answer_to_id: None,
            topic_id: None,
            creation_date: chrono::Utc::now(),
            external_id: "x".into(),
            is_outgoing: true,
        }];
        assert!(format_messages_for_extraction(&msgs).is_empty());
    }
}
