-- openhaze (Rust port) — consolidated initial schema.
--
-- This mirrors the final state of the Django migration graph (crm 0001–0015,
-- core/emails/chat/legacy) as a single clean-room schema. Table and column names
-- follow Django's <app_label>_<model> convention so the data model reads 1:1
-- against openhaze/*/models.py. SQLite is the store (data/db.sqlite3), same
-- as the original (openhaze/settings.py).
--
-- Datetimes are stored as TEXT in RFC3339/ISO-8601 UTC (Django's SQLite datetime
-- representation); booleans as INTEGER 0/1; JSONField/BinaryField as TEXT/BLOB.

-- ---------------------------------------------------------------------------
-- Django auth user (the operator). Only the columns the engine reads are kept:
-- self-hosted means exactly one staff operator (see core/session.get_active_user).
-- ---------------------------------------------------------------------------
CREATE TABLE auth_user (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT    NOT NULL UNIQUE,
    email         TEXT    NOT NULL DEFAULT '',
    password      TEXT    NOT NULL DEFAULT '',
    first_name    TEXT    NOT NULL DEFAULT '',
    last_name     TEXT    NOT NULL DEFAULT '',
    is_staff      INTEGER NOT NULL DEFAULT 0,
    is_active     INTEGER NOT NULL DEFAULT 1,
    is_superuser  INTEGER NOT NULL DEFAULT 0,
    last_login    TEXT,
    date_joined   TEXT    NOT NULL
);

-- ---------------------------------------------------------------------------
-- core.SiteConfig — singleton (pk=1). LLM + provider keys, contacts hub creds,
-- operator country. (openhaze/core/models.py:SiteConfig)
-- ---------------------------------------------------------------------------
CREATE TABLE core_siteconfig (
    id                    INTEGER PRIMARY KEY,          -- forced to 1 by SiteConfig.load()
    ai_model              TEXT NOT NULL DEFAULT '',
    llm_api_key           TEXT NOT NULL DEFAULT '',
    llm_api_base          TEXT NOT NULL DEFAULT '',
    bettercontact_api_key TEXT NOT NULL DEFAULT '',
    contacts_api_token    TEXT NOT NULL DEFAULT '',
    contacts_api_url      TEXT NOT NULL DEFAULT '',
    country_code          TEXT NOT NULL DEFAULT ''
);

-- ---------------------------------------------------------------------------
-- core.Campaign (openhaze/core/models.py:Campaign)
-- ---------------------------------------------------------------------------
CREATE TABLE core_campaign (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL UNIQUE,
    product_docs    TEXT    NOT NULL DEFAULT '',
    campaign_target TEXT    NOT NULL DEFAULT '',
    booking_link    TEXT    NOT NULL DEFAULT '',
    is_freemium     INTEGER NOT NULL DEFAULT 0,
    action_fraction REAL    NOT NULL DEFAULT 0.2,
    seed_public_ids TEXT    NOT NULL DEFAULT '[]',      -- JSONField(default=list)
    model_blob      BLOB,                               -- per-campaign GP (joblib in Python; bincode/serde here)
    country_code    TEXT    NOT NULL DEFAULT ''
);
