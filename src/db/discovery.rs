//! DiscoveryQuery + EmptyClauseSet persistence — the frontier's storage layer.
//! (`frontier.persist_fetched` / `mark_exhausted` / `record_empty` / `node_stats`,
//! and the `descend` / `mutate` read queries.)

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
\n// revival 2026 touch: src/db/discovery.rs\n