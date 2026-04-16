//! ICP seed generator — LLM maps a campaign to its first Lead Finder query and its
//! clause pool. (`pipeline/icp.py`)

use anyhow::Result;
use owo_colors::{AnsiColors, OwoColorize};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::discovery::{describe_clauses, LEAD_SENIORITIES};
use crate::llm::{self, LlmConfig};
use crate::models::Campaign;
use crate::prompts;

fn default_headcount_min() -> i64 {
    1
}
fn default_headcount_max() -> i64 {
    10000
}

/// The LLM's provider-agnostic ICP output — candidate values per family. (`ICPSpec`)
#[derive(Debug, Deserialize, Default)]
pub struct IcpSpec {\n// revival 2026 touch: src/pipeline/icp.rs\n