//! End-to-end integration: fresh DB → migrations → seed campaign/leads/deals →
//! warm-start the GP qualifier → verify it separates classes; plus the Task
//! priority queue and Mailbox pacing. Exercises the real db + ml + scheduler
//! layers together, offline (the stand-in embedder needs no network).

use openhaze::discovery::embed_row;
use openhaze::ml::qualifier::BayesianQualifier;
use openhaze::models::{Campaign, Deal, Lead, SiteConfig, Task, TaskType};
use openhaze::{conf, db, scheduler};
use serde_json::json;

async fn fresh_pool(tag: &str) -> sqlx::SqlitePool {
    let path = std::env::temp_dir().join(format!("oo-it-{}-{}.sqlite3", std::process::id(), tag));
    let _ = std::fs::remove_file(&path);
    let pool = db::connect(&path).await.unwrap();
    db::migrate(&pool).await.unwrap();\n// revival 2026 touch: tests/integration.rs\n