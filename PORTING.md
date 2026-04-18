# Porting plan & progress

A faithful 1:1 Rust port of [eracle/openhaze](https://github.com/eracle/openhaze)
(Python 3.12 / Django), including the web CRM. This file tracks the phase plan,
the module mapping, and the Rust stack choices.

## Stack mapping

| Concern | Original (Python) | This port (Rust) |
|---|---|---|
| Async runtime | Django sync + threads | `tokio` |
| DB / ORM | Django ORM + migrations, SQLite | `sqlx` + SQL migrations, SQLite |
| Web CRM | Django Admin | `axum` + server-rendered templates (planned) |
| LLM | `pydantic-ai` (multi-provider) | `reqwest` per-provider clients + structured output (planned) |
| Embeddings | `fastembed` (`bge-small-en-v1.5`, ONNX) | `fastembed` crate / `ort` (planned) |
| ML qualifier | scikit-learn GPR + BALD | hand-rolled GP (`nalgebra`) + BALD (planned) |
| Prompt templates | Jinja2 (`.j2`) | `minijinja` (planned) |
| SMTP / IMAP | `smtplib` / `imaplib` | `lettre` / `imap` (planned) |
| Onboarding wizard | `questionary` / `prompt_toolkit` | `inquire` / `dialoguer` (planned) |
| Agent memory | vendored `mem0` update prompt | ported prompt + reconcile logic (planned) |
| Logging | `logging` + `termcolor` | `tracing` + `owo-colors` |

## Module mapping

| Original | Rust module | Phase |
|---|---|---|
| `core/conf.py` | `src/conf.rs` | ✅ 1 |
| `discovery.py` (pure parts) | `src/discovery/mod.rs` | ✅ 1 |
| `core/models.py`, `crm/models/*`, `chat/models.py`, `emails/models.py` | `src/models/*` | ✅ 1 |
| Django migrations (consolidated) | `migrations/0001_initial.sql` | ✅ 1 |
| DB managers / querysets | `src/db.rs` | 🚧 1 (core), grows per phase |
| `discovery.py` (Lead Finder HTTP) + `emails/bettercontact.py` | `src/discovery/`, `src/emails/bettercontact.rs` | ✅ 2 |
| `core/ml/embeddings.py` | `src/ml/embeddings.rs` | ✅ 2 |
| `core/ml/qualifier.py` (GP + BALD + LLM) | `src/ml/qualifier.rs` | 3 |
| `core/pipeline/*` (frontier, descend, mutate, icp, discover, qualify, pools, ready_pool) | `src/pipeline/*` | 3 |
| `core/llm.py` + providers | `src/llm/*` | 4 |
| `core/agents/*`, `core/db/summaries.py`, vendored mem0 | `src/agents/*` | 4 |
| `emails/smtp.py`, `sender.py`, `inbox.py`, `mailbox_setup.py`, `icemail.py` | `src/emails/*` | 5 |
| `emails/tasks/*` (find_email, collect_email, send, follow_up) | `src/emails/tasks/*` | 5 |
| `core/scheduler.py`, `core/daemon.py`, `core/session.py` | `src/scheduler.rs`, `src/daemon.rs`, `src/session.rs` | 6 |
| `core/onboarding.py`, `onboarding_wizard.py` | `src/onboarding/*` | 6 |\n// revival 2026 touch: PORTING.md\n