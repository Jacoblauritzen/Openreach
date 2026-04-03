//! Database layer — the SQLite store (`data/db.sqlite3`) and the query helpers
//! that back the Django managers/querysets. Uses `sqlx` with runtime-checked
//! queries (no `DATABASE_URL` needed at build time).

use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};

use crate::models::{Mailbox, SiteConfig, Task, TaskStatus};