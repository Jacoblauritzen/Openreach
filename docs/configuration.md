# Configuration

Two DB rows hold everything an operator sets; both are editable in the web CRM.

## Site Configuration (`/config`)

A singleton row (`core_siteconfig`).

| Field | Meaning |
|---|---|
| `ai_model` | LLM as `provider:model`, e.g. `anthropic:claude-sonnet-4-5`, `openai:gpt-4o`, `groq:llama-3.3-70b`. A bare `gpt-*`/`o1`/`o3`/`claude-*`/`gemini-*` is also accepted. |
| `llm_api_key` | Key for the chosen provider. |
| `llm_api_base` | Only for `openai_compatible:*` (OpenRouter / Together / Ollama / vLLM). |
| `bettercontact_api_key` | Powers **both** discovery (Lead Finder) and enrichment; blank disables both. |
| `contacts_api_token` / `contacts_api_url` | The shared contacts hub — token earned on first give-back; blank URL falls back to the default hub. |
| `country_code` | Your ISO-3166 alpha-2 country. Drives the active-hours timezone and the email-jurisdiction rules. |

**Providers:** `openai`, `anthropic`, `google`, `groq`, `mistral`, `cohere`, `openai_compatible`.

## Campaign (`/campaigns/:id`)

One row per campaign (`core_campaign`).

| Field | Meaning |
|---|---|
| `product_docs` | What you sell — fed to the ICP, qualifier, and outreach prompts. |
| `campaign_target` | Who you're targeting (the objective). |
| `booking_link` | Optional; offered by the agent when a lead wants to meet. |
| `country_code` | The ICP's target country, stamped on discovered leads for the geo-gate. |
| *(learned state)* | The clause pool, discovery-query nodes, and the serialized GP model — managed by the engine. |

## Static defaults (`src/conf.rs`)

Compile-time timing + ML knobs (no YAML):

| Constant | Default | Meaning |
|---|---|---|
| `DEFAULT_EMAIL_DAILY_LIMIT` | 30 | Warm-safe sends/day, per mailbox. |
| `MIN_GP_CONFIDENCE` | 0.9 | GP gate — `P(f>0.5)` above this promotes a lead to the paid lookup. |
| `EMBEDDING_MODEL` | `BAAI/bge-small-en-v1.5` | 384-dim embeddings. |
| `ENABLE_ACTIVE_HOURS` | `false` | Run 24/7; set true for a 09:00–19:00 local window. |
| `COLLECT_BACKOFF_BASE_S` / `_MAX_S` / `COLLECT_DEADLINE_S` | 5 / 60 / 600 | Poll backoff for an in-flight email lookup. |

## Environment

| Var | Meaning |
|---|---|
| `OPENREACH_ROOT` | Project root that holds `data/` (defaults to the working directory). |
| `RUST_LOG` | Log level, e.g. `info` or `openhaze=debug`. |
