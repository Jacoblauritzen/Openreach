//! Data models — a port of the Django models across the `core`, `crm`, `chat`,
//! and `emails` apps. Each struct mirrors one table (see `migrations/`), field
//! for field. Enum-valued columns are stored as TEXT and exposed through typed
//! accessors (see `deal::DealState`, `task::TaskType`, …).

pub mod campaign;
pub mod chat;
pub mod deal;
pub mod lead;
pub mod mailbox;
pub mod site_config;
pub mod task;
pub mod user;

pub use campaign::{Campaign, Clause, DiscoveryQuery, EmptyClauseSet};
pub use chat::ChatMessage;
pub use deal::{Deal, DealState, Outcome};
pub use lead::Lead;
pub use mailbox::Mailbox;
pub use site_config::SiteConfig;
pub use task::{Task, TaskStatus, TaskType};
pub use user::User;

use sha2::{Digest, Sha256};

/// Deterministic text for a clause set — sorted `family=value` pairs rendered as
/// compact JSON. Mirrors `frontier.canonicalize`:
/// `json.dumps(sorted(clauses), separators=(",", ":"))`.
///
/// Note: Python's `json.dumps` defaults to `ensure_ascii=True` (non-ASCII → `\uXXXX`);
/// `serde_json` emits UTF-8. The hash is only ever compared against other hashes
/// this program produces, so the representation only needs to be internally
/// deterministic — which it is.
pub fn canonicalize_clauses(clauses: &[(String, String)]) -> String {
    let mut sorted: Vec<[&str; 2]> = clauses.iter().map(|(f, v)| [f.as_str(), v.as_str()]).collect();
    sorted.sort();
    serde_json::to_string(&sorted).expect("clause set serializes")
}

/// sha256 of the canonicalized clause set — the node-identity key for dedup,
/// order-independent. Mirrors `frontier.clause_key`.
pub fn clause_key(clauses: &[(String, String)]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(canonicalize_clauses(clauses).as_bytes());
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clause_key_is_order_independent() {
        let a = vec![
            ("lead_job_title".to_string(), "Founder".to_string()),
            ("lead_location".to_string(), "United States".to_string()),
        ];
        let mut b = a.clone();
        b.reverse();
        assert_eq!(clause_key(&a), clause_key(&b));
    }

    #[test]
    fn canonicalize_matches_python_shape() {
        let clauses = vec![
            ("lead_job_title".to_string(), "Founder".to_string()),
            ("company_headcount_min".to_string(), "1".to_string()),
        ];
        // sorted → company_headcount_min before lead_job_title; compact JSON arrays.
        assert_eq!(
            canonicalize_clauses(&clauses),
            r#"[["company_headcount_min","1"],["lead_job_title","Founder"]]"#
        );
    }
}
