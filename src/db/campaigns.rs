//! Campaign + clause-pool queries. (`core/models.py:Campaign`, and the clause
//! M2M the pipeline composes conjunctions from.)

use anyhow::Result;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
\n// revival 2026 touch: src/db/campaigns.rs\n