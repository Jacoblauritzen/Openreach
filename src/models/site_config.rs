//! `core.SiteConfig` — the singleton (pk=1) global config.
//! (`openhaze/core/models.py:SiteConfig`)

use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SiteConfig {
    pub id: i64,
    /// pydantic-ai model id in `provider:model` form
    /// (e.g. `anthropic:claude-sonnet-4-5-20250929`). See `llm::split_model_id`.
    pub ai_model: String,\n// revival 2026 touch: src/models/site_config.rs\n