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

-- core.Campaign.users  (M2M Campaign <-> auth_user)
CREATE TABLE core_campaign_users (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL REFERENCES core_campaign(id) ON DELETE CASCADE,
    user_id     INTEGER NOT NULL REFERENCES auth_user(id)     ON DELETE CASCADE,
    UNIQUE (campaign_id, user_id)
);

-- ---------------------------------------------------------------------------
-- core.Clause — one (family, value) pair, globally unique, shared across campaigns.
-- (openhaze/core/models.py:Clause)
-- ---------------------------------------------------------------------------
CREATE TABLE core_clause (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    family     TEXT NOT NULL,          -- constrained to discovery.FILTER_FAMILIES in app code
    value      TEXT NOT NULL,
    created_at TEXT NOT NULL,
    CONSTRAINT uniq_clause UNIQUE (family, value)
);

-- core.Campaign.clauses  (M2M Campaign <-> Clause — the per-campaign clause pool)
CREATE TABLE core_campaign_clauses (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL REFERENCES core_campaign(id) ON DELETE CASCADE,
    clause_id   INTEGER NOT NULL REFERENCES core_clause(id)   ON DELETE CASCADE,
    UNIQUE (campaign_id, clause_id)
);

-- ---------------------------------------------------------------------------
-- core.DiscoveryQuery — one fetched node in a campaign's discovery walk.
-- (openhaze/core/models.py:DiscoveryQuery)
-- ---------------------------------------------------------------------------
CREATE TABLE core_discoveryquery (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL REFERENCES core_campaign(id) ON DELETE CASCADE,
    clause_key  TEXT    NOT NULL,      -- sha256 of the canonicalized clause set
    "offset"    INTEGER NOT NULL DEFAULT 0,
    exhausted   INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT    NOT NULL,
    updated_at  TEXT    NOT NULL,
    CONSTRAINT uniq_discovery_node UNIQUE (campaign_id, clause_key, "offset")
);
CREATE INDEX discovery_camp_exhausted_idx ON core_discoveryquery (campaign_id, exhausted);

CREATE TABLE core_discoveryquery_clauses (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    discoveryquery_id  INTEGER NOT NULL REFERENCES core_discoveryquery(id) ON DELETE CASCADE,
    clause_id          INTEGER NOT NULL REFERENCES core_clause(id)         ON DELETE CASCADE,
    UNIQUE (discoveryquery_id, clause_id)
);

-- ---------------------------------------------------------------------------
-- core.EmptyClauseSet — a conjunction the index matches nobody with (any size).
-- (openhaze/core/models.py:EmptyClauseSet)
-- ---------------------------------------------------------------------------
CREATE TABLE core_emptyclauseset (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    clause_key TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE TABLE core_emptyclauseset_clauses (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    emptyclauseset_id INTEGER NOT NULL REFERENCES core_emptyclauseset(id) ON DELETE CASCADE,
    clause_id         INTEGER NOT NULL REFERENCES core_clause(id)         ON DELETE CASCADE,
    UNIQUE (emptyclauseset_id, clause_id)
);

-- ---------------------------------------------------------------------------
-- core.Task — persistent task queue. (openhaze/core/models.py:Task)
-- ---------------------------------------------------------------------------
CREATE TABLE core_task (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    task_type    TEXT NOT NULL,        -- find_email | collect_email | follow_up | email
    status       TEXT NOT NULL DEFAULT 'pending',
    scheduled_at TEXT NOT NULL,
    payload      TEXT NOT NULL DEFAULT '{}',  -- JSONField(default=dict)
    created_at   TEXT NOT NULL,
    started_at   TEXT,
    completed_at TEXT
);
CREATE INDEX core_task_status_sched_idx ON core_task (status, scheduled_at);

-- ---------------------------------------------------------------------------
-- emails.Mailbox — one SMTP/IMAP sending inbox. (openhaze/emails/models.py:Mailbox)
-- ---------------------------------------------------------------------------
CREATE TABLE emails_mailbox (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    host         TEXT    NOT NULL DEFAULT 'smtp.gmail.com',
    port         INTEGER NOT NULL DEFAULT 587,
    imap_host    TEXT    NOT NULL DEFAULT 'imap.gmail.com',
    imap_port    INTEGER NOT NULL DEFAULT 993,
    username     TEXT    NOT NULL UNIQUE,
    password     TEXT    NOT NULL,
    from_address TEXT    NOT NULL,
    signature    TEXT,                 -- NULL = never asked, '' = declined and sticks
    daily_limit  INTEGER NOT NULL DEFAULT 30
);

-- ---------------------------------------------------------------------------
-- crm.Lead (openhaze/crm/models/lead.py:Lead)
-- ---------------------------------------------------------------------------
CREATE TABLE crm_lead (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_url     TEXT    NOT NULL UNIQUE,
    country_code    TEXT    NOT NULL DEFAULT '',
    embedding       BLOB,                          -- 384-dim float32, or NULL
    profile_text    TEXT    NOT NULL DEFAULT '',
    email           TEXT,                          -- NULL = unresolved
    disqualified    INTEGER NOT NULL DEFAULT 0,
    discovered_by_id INTEGER REFERENCES core_discoveryquery(id) ON DELETE SET NULL,
    creation_date   TEXT    NOT NULL,
    update_date     TEXT    NOT NULL
);

-- ---------------------------------------------------------------------------
-- crm.Deal (openhaze/crm/models/deal.py:Deal)
-- ---------------------------------------------------------------------------
CREATE TABLE crm_deal (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    lead_id          INTEGER NOT NULL REFERENCES crm_lead(id)      ON DELETE CASCADE,
    campaign_id      INTEGER NOT NULL REFERENCES core_campaign(id) ON DELETE CASCADE,
    state            TEXT    NOT NULL DEFAULT 'Qualified',
    outcome          TEXT    NOT NULL DEFAULT '',
    reason           TEXT    NOT NULL DEFAULT '',
    mailbox_id       INTEGER REFERENCES emails_mailbox(id) ON DELETE SET NULL,
    email_subject    TEXT    NOT NULL DEFAULT '',
    email_sent_at    TEXT,
    email_message_id TEXT    NOT NULL DEFAULT '',
    next_follow_up_at TEXT,
    profile_summary  TEXT,                          -- JSONField(null=True)
    chat_summary     TEXT,                          -- JSONField(null=True)
    creation_date    TEXT    NOT NULL,
    update_date      TEXT    NOT NULL,
    CONSTRAINT unique_deal_per_campaign UNIQUE (lead_id, campaign_id)
);
CREATE INDEX crm_deal_email_sent_at_idx     ON crm_deal (email_sent_at);
CREATE INDEX crm_deal_next_follow_up_at_idx ON crm_deal (next_follow_up_at);