//! Find-email pool: the GP confidence gate between QUALIFIED and
//! READY_TO_FIND_EMAIL — the spend gate on the paid lookup. (`pipeline/ready_pool.py`)

use anyhow::Result;
use sqlx::SqlitePool;

use crate::db::Profile;
use crate::ml::qualifier::BayesianQualifier;
use crate::models::deal::DealState;
use crate::models::{Deal, Lead};

/// Load `(profile, embedding)` pairs for profiles that have an embedding.
async fn with_embeddings(
    db: &SqlitePool,
    profiles: Vec<Profile>,
) -> Result<(Vec<Profile>, Vec<Vec<f32>>)> {
    let mut valid = Vec::new();
    let mut embs = Vec::new();
    for p in profiles {
        if let Some(lead) = Lead::get(db, p.lead_id).await? {
            if let Some(e) = lead.embedding_array() {
                embs.push(e);
                valid.push(p);
            }
        }
    }
    Ok((valid, embs))
}

/// Promote QUALIFIED profiles scoring above the GP threshold to
/// READY_TO_FIND_EMAIL. Returns the count promoted. (`promote_to_ready`)
pub async fn promote_to_ready(
    db: &SqlitePool,
    campaign_id: i64,