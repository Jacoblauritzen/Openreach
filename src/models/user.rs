//! The operator — Django's `auth_user`. Self-hosted means one staff operator
//! (`core/session.get_active_user`); identity (email/name) lives here while the
//! operational config lives on `SiteConfig`.

use chrono::{DateTime, Utc};