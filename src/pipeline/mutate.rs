//! The frontier's breadth move — one new Lead Finder query, composed or invented.
//! (`pipeline/mutate.py`)
//!
//! `descend_or_refill`: compose the next untried conjunction from the clause pool
//! (`descend`), and ask the LLM only when the pool spans nothing new.

use anyhow::Result;
use owo_colors::{AnsiColors, OwoColorize};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sqlx::SqlitePool;

use crate::discovery::{describe_clauses, describe_filters, LEAD_SENIORITIES};
use crate::llm::{self, LlmConfig};
use crate::models::{Campaign, DiscoveryQuery};
use crate::pipeline::descend;
use crate::prompts;

/// Cap on past queries listed in the prompt. (`PAST_QUERY_LIMIT`)
const PAST_QUERY_LIMIT: i64 = 40;

/// The LLM's proposed filter set (only families verified to steer). (`_Filters`)
#[derive(Debug, Deserialize, Default)]
struct Filters {
    company_headcount_min: Option<i64>,
    company_headcount_max: Option<i64>,
    lead_job_title: Option<String>,
    lead_seniority: Option<String>,
    lead_location: Option<String>,
    lead_department: Option<String>,
    lead_function: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct FilterSet {
    #[serde(default)]
    filters: Filters,
}

impl Filters {
    /// Set fields as sorted `(family, value)` pairs (None dropped). (`model_dump(exclude_none)`)
    fn to_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(v) = self.company_headcount_min {
            pairs.push(("company_headcount_min".to_string(), v.to_string()));
        }
        if let Some(v) = self.company_headcount_max {
            pairs.push(("company_headcount_max".to_string(), v.to_string()));
        }
        if let Some(v) = &self.lead_job_title {
            pairs.push(("lead_job_title".to_string(), v.clone()));
        }
        if let Some(v) = &self.lead_seniority {
            pairs.push(("lead_seniority".to_string(), v.clone()));
        }
        if let Some(v) = &self.lead_location {
            pairs.push(("lead_location".to_string(), v.clone()));
        }
        if let Some(v) = &self.lead_department {
            pairs.push(("lead_department".to_string(), v.clone()));
        }
        if let Some(v) = &self.lead_function {
            pairs.push(("lead_function".to_string(), v.clone()));
        }
        pairs.sort();
        pairs
    }
}

/// Recent fetched nodes with their measured value, newest first. (`_past_query_stats`)
async fn past_query_stats(db: &SqlitePool, campaign_id: i64) -> Result<Vec<Value>> {
    let nodes = DiscoveryQuery::recent_for_campaign(db, campaign_id, PAST_QUERY_LIMIT).await?;
    let stats = DiscoveryQuery::node_stats(db, campaign_id).await?;
    let mut out = Vec::new();
    for n in nodes {
        let pairs = n.clause_pairs(db).await?;
        let filters = DiscoveryQuery::to_filters(&pairs).unwrap_or(Value::Null);
        let (examined, qualified) = stats.get(&n.id).copied().unwrap_or((0, 0));
        out.push(json!({
            "query": describe_filters(&filters),
            "offset": n.offset,
            "n_leads": DiscoveryQuery::lead_count(db, n.id).await?,
            "examined": examined,
            "qualified": qualified,
        }));
    }
    Ok(out)
}

fn filters_schema() -> Value {
    let mut props = Map::new();
    props.insert("company_headcount_min".into(), json!({"type": ["integer", "null"]}));
    props.insert("company_headcount_max".into(), json!({"type": ["integer", "null"]}));
    props.insert("lead_job_title".into(), json!({"type": ["string", "null"], "description": "One role title, plain — no lists."}));
    props.insert("lead_seniority".into(), json!({"type": ["string", "null"], "enum": senior_enum_with_null()}));
    props.insert("lead_location".into(), json!({"type": ["string", "null"], "description": "One country by real name; regions match nothing."}));
    props.insert("lead_department".into(), json!({"type": ["string", "null"], "description": "One department, plain name e.g. 'Sales'."}));
    props.insert("lead_function".into(), json!({"type": ["string", "null"], "description": "One broad function, plain name."}));
    json!({
        "type": "object",
        "properties": {"filters": {"type": "object", "properties": props, "additionalProperties": false}},
    })
}

fn senior_enum_with_null() -> Vec<Value> {
    let mut v: Vec<Value> = LEAD_SENIORITIES.iter().map(|s| json!(s)).collect();