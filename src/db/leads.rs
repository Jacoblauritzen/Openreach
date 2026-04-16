//! Lead persistence + labelled-array extraction. (`core/db/leads.py`,
//! `Lead.get_labeled_arrays`.)

use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use serde_json::Value;
use sqlx::{Row, SqlitePool};

use crate::discovery;
use crate::models::deal::DealState;
use crate::models::Lead;

const LEAD_COLUMNS: &str = "id, profile_url, country_code, embedding, profile_text, email, \
     disqualified, discovered_by_id, creation_date, update_date";

impl Lead {
    pub async fn get_by_url(pool: &SqlitePool, profile_url: &str) -> Result<Option<Lead>> {
        let sql = format!("SELECT {LEAD_COLUMNS} FROM crm_lead WHERE profile_url = ?1");
        Ok(sqlx::query_as::<_, Lead>(&sql)
            .bind(profile_url)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Lead>> {
        let sql = format!("SELECT {LEAD_COLUMNS} FROM crm_lead WHERE id = ?1");
        Ok(sqlx::query_as::<_, Lead>(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await?)
    }

    /// Persist one Lead Finder row as an embedded Lead awaiting qualification.
    /// Keyed on `contact_linkedin_profile_url`. Returns `true` when a new Lead was
    /// created, `false` when one already existed. (`create_lead`)
    pub async fn create_from_row(
        pool: &SqlitePool,
        row: &Value,
        country_code: &str,
        discovered_by_id: Option<i64>,
    ) -> Result<bool> {
        let Some(profile_url) = row
            .get("contact_linkedin_profile_url")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        else {
            return Ok(false);
        };

        if Lead::get_by_url(pool, profile_url).await?.is_some() {
            return Ok(false); // idempotent re-discovery
        }

        let embedding = Lead::embedding_bytes(&discovery::embed_row(row));
        let profile_text = discovery::profile_text_for(row);
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO crm_lead
                (profile_url, country_code, embedding, profile_text, discovered_by_id,
                 creation_date, update_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
        )
        .bind(profile_url)
        .bind(country_code)
        .bind(embedding)
        .bind(profile_text)
        .bind(discovered_by_id)
        .bind(&now)
        .execute(pool)
        .await?;
        Ok(true)
    }

    /// Set `Lead.disqualified = true` (account-level, permanent). (`disqualify_lead`)
    pub async fn disqualify(pool: &SqlitePool, profile_url: &str) -> Result<()> {
        let res = sqlx::query("UPDATE crm_lead SET disqualified = 1 WHERE profile_url = ?1")
            .bind(profile_url)
            .execute(pool)
            .await?;
        if res.rows_affected() == 0 {
            tracing::warn!("disqualify_lead: no Lead for {profile_url}");
        }
        Ok(())
    }

    /// Store the resolved work email on the lead.
    pub async fn set_email(pool: &SqlitePool, lead_id: i64, email: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE crm_lead SET email = ?2, update_date = ?3 WHERE id = ?1")
            .bind(lead_id)
            .bind(email)
            .bind(now)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Embedded, un-dealt, non-disqualified Leads awaiting qualification, oldest
    /// first. (`qualify.fetch_qualification_candidates`)
    pub async fn qualification_candidates(
        pool: &SqlitePool,
        campaign_id: i64,
    ) -> Result<Vec<Lead>> {
        let sql = format!(
            "SELECT {LEAD_COLUMNS} FROM crm_lead l
             WHERE l.disqualified = 0 AND l.embedding IS NOT NULL
               AND NOT EXISTS (SELECT 1 FROM crm_deal d WHERE d.lead_id = l.id AND d.campaign_id = ?1)
             ORDER BY l.creation_date"
        );
        Ok(sqlx::query_as::<_, Lead>(&sql)
            .bind(campaign_id)
            .fetch_all(pool)
            .await?)
    }

    /// Labelled embeddings for a campaign as `(X, y)` for GP warm start.
    /// label 1 = non-FAILED deal; label 0 = FAILED+wrong_fit; others skipped.
    /// (`Lead.get_labeled_arrays`)
    pub async fn get_labeled_arrays(
        pool: &SqlitePool,
        campaign_id: i64,
    ) -> Result<(Vec<Vec<f32>>, Vec<i32>)> {
        let rows = sqlx::query(
            "SELECT lead_id, state, outcome FROM crm_deal WHERE campaign_id = ?1",
        )
        .bind(campaign_id)
        .fetch_all(pool)
        .await?;

        let mut label_by_lead: HashMap<i64, i32> = HashMap::new();
        for r in rows {
            let lid: i64 = r.get("lead_id");
            let state: String = r.get("state");
            let outcome: String = r.get("outcome");
            if state == DealState::Failed.as_str() {
                if outcome == "wrong_fit" {
                    label_by_lead.insert(lid, 0);
                }
            } else {
                label_by_lead.insert(lid, 1);
            }
        }
        if label_by_lead.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        let mut x = Vec::new();
        let mut y = Vec::new();
        // Deterministic order by lead id.
        let mut ids: Vec<i64> = label_by_lead.keys().copied().collect();
        ids.sort_unstable();
        for lid in ids {
            if let Some(lead) = Lead::get(pool, lid).await? {
                if let Some(emb) = lead.embedding_array() {
                    x.push(emb);
                    y.push(label_by_lead[&lid]);
                }
            }
        }
        Ok((x, y))
    }
}
