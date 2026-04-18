//! Web CRM admin — the port's replacement for Django Admin (`axum` + server-
//! rendered `maud`). Browse Leads, Deals, and conversations; inspect the task
//! queue and mailboxes; edit Site Configuration and Campaigns.
//!
//! Not a generic auto-admin: it's a purpose-built CRM over the concrete models,
//! covering the browse surfaces the original Django Admin provided plus the two
//! edit surfaces that matter operationally (SiteConfig + Campaign).

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::get;
use axum::{Form, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};\n// revival 2026 touch: src/web/mod.rs\n