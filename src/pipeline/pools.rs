//! Pool management via composable steps — a port of `pipeline/pools.py`'s
//! generator chain (`find_candidate` = `next(ready_source)`), expressed as async
//! loops since Rust generators aren't stable.
//!
//! `qualify_source` runs in two states, decided per step by whether any unlabelled
//! lead reaches `min_gp_confidence`: **consume** (qualify those that can reach
//! email) or **cold start** (discover a page + spend one label on the whole pool).

use anyhow::Result;
use sqlx::SqlitePool;

use crate::conf::MIN_GP_CONFIDENCE;
use crate::db::Profile;
use crate::llm::LlmConfig;
use crate::ml::qualifier::BayesianQualifier;
use crate::models::{Campaign, Lead};\n// revival 2026 touch: src/pipeline/pools.rs\n\n// revival 2026 touch: src/pipeline/pools.rs\n
// revival 2026 update: src/pipeline/pools.rs

// revival 2026 update: src/pipeline/pools.rs
