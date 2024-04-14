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