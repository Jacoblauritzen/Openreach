//! openhaze (Rust port) — a self-hosted, email-first AI sales agent.
//!
//! A faithful clean-room reimplementation of eracle/openhaze (GPLv3). The
//! module layout mirrors the original's `openhaze/` package:
//!
//! - [`conf`]       — static timing + ML config (`core/conf.py`)
//! - [`discovery`]  — Lead Finder filter vocabulary + rendering (`discovery.py`)
//! - [`models`]     — the Django models across `core` / `crm` / `chat` / `emails`
//! - [`db`]         — the SQLite store and the manager/queryset query helpers
//!
//! Later phases add the pipeline (discover → qualify → find-email → email →
//! follow-up), the ML qualifier (GP + BALD), the LLM + embedding + SMTP/IMAP
//! clients, the daemon/scheduler, onboarding, and the web CRM.
//!
//! The freemium self-promo campaign and auto-newsletter behavior of the original
//! are intentionally **omitted** — 100% of sending is the operator's own outreach.

pub mod agents;
pub mod conf;
pub mod contacts;
pub mod daemon;
pub mod db;
pub mod discovery;