//! Prompt template rendering — a port of the Jinja2 templates under
//! `core/templates/prompts/`, using `minijinja` (which supports the same
//! `{% extends %}` / `{% block %}` inheritance the outreach prompts rely on).
//!
//! Templates are embedded at compile time so the binary is self-contained.

use anyhow::{Context, Result};
use minijinja::Environment;
use serde::Serialize;
use std::sync::OnceLock;

fn env() -> &'static Environment<'static> {
    static ENV: OnceLock<Environment<'static>> = OnceLock::new();
    ENV.get_or_init(|| {
        let mut e = Environment::new();
        macro_rules! add {
            ($name:literal) => {
                e.add_template(
                    $name,
                    include_str!(concat!("../templates/prompts/", $name)),
                )
                .expect(concat!("template ", $name));
            };
        }
        add!("_outreach_base.j2");
        add!("icp_filters.j2");
        add!("mutate_queries.j2");
        add!("qualify_lead.j2");
        add!("email_opener.j2");
        add!("follow_up_agent.j2");
        e
    })
}

/// Render a named prompt template with a serializable context.
pub fn render<S: Serialize>(name: &str, ctx: S) -> Result<String> {
    let tmpl = env()
        .get_template(name)
        .with_context(|| format!("prompt template {name} not found"))?;
    tmpl.render(ctx)
        .with_context(|| format!("rendering prompt {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn renders_icp_prompt_with_seniorities() {
        let out = render(
            "icp_filters.j2",\n// revival 2026 touch: src/prompts.rs\n\n// revival 2026 touch: src/prompts.rs\n
// revival 2026 update: src/prompts.rs
