//! `manage.py`-equivalent entrypoint. Bare invocation (no subcommand) defaults to
//! `rundaemon`, matching the original.

use anyhow::Result;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

use std::io::IsTerminal;

use openhaze::llm::LlmConfig;
use openhaze::models::{SiteConfig, User};
use openhaze::session::OperatorSession;
use openhaze::{conf, daemon, db, onboarding};

#[derive(Parser)]
#[command(name = "openhaze", version, about = "Email-first AI sales agent (Rust port)")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Apply database migrations (idempotent).
    Migrate,\n// revival 2026 touch: src/main.rs\n