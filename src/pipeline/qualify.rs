//! Qualify orchestration — pick one unlabelled lead, ask the LLM, save the verdict.
//! (`pipeline/qualify.py`, and `qualifier.qualify_with_llm`.)

use anyhow::Result;
use owo_colors::{AnsiColors, OwoColorize};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::llm::{self, LlmConfig};
use crate::ml::qualifier::{format_prediction, BayesianQualifier};
use crate::models::{Campaign, Deal, Lead};
use crate::prompts;

/// Structured LLM output for lead qualification. (`QualificationDecision`)
#[derive(Debug, Deserialize)]
struct QualificationDecision {
    qualified: bool,
    #[serde(default)]
    reason: String,
}

/// Call the LLM to qualify a profile. Returns `(label, reason)`; label 1 = accept.
/// (`qualify_with_llm`)
pub async fn qualify_with_llm(
    llm: &LlmConfig,
    profile_text: &str,
    product_docs: &str,
    campaign_target: &str,
) -> Result<(i32, String)> {
    let prompt = prompts::render(
        "qualify_lead.j2",
        json!({
            "product_docs": product_docs,
            "campaign_target": campaign_target,
            "profile_text": profile_text,
        }),
    )?;
    let schema = json!({
        "type": "object",
        "properties": {
            "qualified": {"type": "boolean", "description": "True if a good prospect."},
            "reason": {"type": "string", "description": "Brief explanation."}
        },
        "required": ["qualified", "reason"]
    });
    let decision: QualificationDecision = llm::complete_structured(llm, &prompt, &schema, 0.7).await?;
    Ok((if decision.qualified { 1 } else { 0 }, decision.reason))
}

/// Qualify one unlabelled profile via the LLM. Returns `profile_url` or `None`.
/// (`run_qualification`)
pub async fn run_qualification(
    db: &SqlitePool,
    llm: &LlmConfig,
    campaign: &Campaign,
    qualifier: &mut BayesianQualifier,
    candidates: Option<Vec<Lead>>,
) -> Result<Option<String>> {
    let candidates = match candidates {
        Some(c) => c,
        None => Lead::qualification_candidates(db, campaign.id).await?,
    };
    if candidates.is_empty() {
        return Ok(None);
    }

    tracing::info!("{}", "▶ qualify".color(AnsiColors::Blue).bold());

    // Balance-driven candidate selection.
    let candidate = if candidates.len() == 1 {
        &candidates[0]
    } else {
        let embeddings: Vec<Vec<f32>> = candidates
            .iter()
            .filter_map(|c| c.embedding_array())
            .collect();
        match qualifier.acquisition_scores(&embeddings) {
            Some((strategy, scores)) => {
                let best = argmax(&scores);
                let (n_neg, n_pos) = qualifier.class_counts();
                tracing::info!(
                    "Strategy: {} (neg={n_neg}, pos={n_pos})",
                    strategy.color(AnsiColors::Cyan).bold()
                );
                &candidates[best]
            }
            None => &candidates[0],
        }
    };

    let profile_url = candidate.profile_url.clone();
    let embedding = candidate.embedding_array().unwrap_or_default();

    if let Some((p, entropy, std)) = qualifier.predict(&embedding) {
        tracing::debug!(
            "{profile_url} ({}) — querying LLM",
            format_prediction(p, entropy, std, qualifier.n_obs())
        );
    } else {
        tracing::debug!("{profile_url} GP not fitted ({} obs) — querying LLM", qualifier.n_obs());
    }

    if candidate.profile_text.is_empty() {
        tracing::debug!("No profile text for {profile_url} — skipping qualification");
        return Ok(None);
    }

    let (label, reason) =
        qualify_with_llm(llm, &candidate.profile_text, &campaign.product_docs, &campaign.campaign_target)
            .await?;
    save_qualification_result(db, campaign, qualifier, &profile_url, &embedding, label, &reason).await?;
    Ok(Some(profile_url))
}

async fn save_qualification_result(
    db: &SqlitePool,
    campaign: &Campaign,
    qualifier: &mut BayesianQualifier,
    profile_url: &str,
    embedding: &[f32],
    label: i32,
    reason: &str,
) -> Result<()> {
    qualifier.update(embedding, label);
    if label == 1 {
        match Deal::promote_lead_to_deal(db, campaign.id, profile_url, reason).await {
            Ok(_) => {
                tracing::info!(
                    "{profile_url} {}: {reason}",
                    "QUALIFIED".color(AnsiColors::Green).bold()
                );
            }
            Err(e) => {
                tracing::warn!("Cannot promote {profile_url}: {e} — disqualifying");
                Deal::create_disqualified_deal(db, campaign.id, profile_url, &e.to_string()).await?;
            }
        }
    } else {
        Deal::create_disqualified_deal(db, campaign.id, profile_url, reason).await?;
    }
    Ok(())
}

fn argmax(scores: &[f64]) -> usize {
    let mut best = 0;
    for (i, s) in scores.iter().enumerate() {
        if s > &scores[best] {
            best = i;
        }
    }
    best
}
\n// revival 2026 touch: src/pipeline/qualify.rs\n