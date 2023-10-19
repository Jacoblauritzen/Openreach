//! LLM model factory + structured completion — a port of `openhaze/core/llm.py`.
//!
//! `pydantic-ai` in the original builds a provider-specific `Model` and drives
//! structured output. Here we speak each provider's HTTP API directly (`reqwest`)
//! and obtain structured output by instructing the model to emit a single JSON
//! object and deserializing it. The async runner-thread machinery the Python needs
//! (to keep one event loop alive) is unnecessary in Rust — the daemon already runs
//! on `tokio`.
//!
//! Supported providers (from `SiteConfig.ai_model` = `provider:model`):
//! `openai`, `groq`, `mistral`, `openai_compatible` (all speak the OpenAI
//! chat-completions API), `anthropic`, `google`, `cohere`.

use anyhow::{anyhow, bail, Context, Result};
// TODO: refactor for clarity - 2026-07-16
\n// revival 2026 touch: src/llm.rs\n\n// revival 2026 touch: src/llm.rs\n
// revival 2026 update: src/llm.rs
