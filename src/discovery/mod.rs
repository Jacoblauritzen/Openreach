//! Lead discovery vocabulary + filter construction — a port of the pure parts of
//! `openhaze/discovery.py`.
//!
//! The live Lead Finder HTTP client (`search`) lands with the enrichment
//! transport in a later phase; this module carries the parts the data model and
//! the frontier depend on: the filter families, the clause→filter-dict mapping,
//! and the human rendering used in logs and `__str__`.

use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

use crate::emails::bettercontact::{self, BetterContactError};
use crate::ml::embeddings;

/// Lead Finder async endpoint. (`discovery.LEAD_FINDER_URL`)
pub const LEAD_FINDER_URL: &str = "https://app.bettercontact.rocks/api/v2/lead_finder/async";

/// Discovery page size — 100 rows per fetch. (`frontier.DISCOVERY_PAGE_SIZE`)
pub const DISCOVERY_PAGE_SIZE: i64 = 100;

/// Lead Finder's `lead_seniority` vocabulary — the only values it matches. An
/// unknown value is not an error: the API returns an empty page, which the
/// frontier reads as end-of-depth. (`discovery.Seniority` / `LEAD_SENIORITIES`)
pub const LEAD_SENIORITIES: [&str; 12] = [
    "owner", "founder", "c_suite", "partner", "vp", "head", "director", "manager",
    "senior", "mid-level", "entry", "intern",
];

/// The filter families a query may be composed from — every one verified to
/// steer. (`discovery.FILTER_FAMILIES`)
pub const FILTER_FAMILIES: [&str; 7] = [
    "company_headcount_min",
    "company_headcount_max",
    "lead_job_title",
    "lead_seniority",
    "lead_location",
    "lead_department",
    "lead_function",
];

/// Families whose value is a bare scalar rather than an `include` list.
/// (`discovery._SCALAR_FAMILIES`)
const SCALAR_FAMILIES: [&str; 2] = ["company_headcount_min", "company_headcount_max"];

/// Lead-row fields we embed, folded in only when the row carries them.
/// (`discovery.TEXT_FIELDS`)
pub const TEXT_FIELDS: [&str; 8] = [
    "contact_headline",
    "contact_industry",
    "contact_job_title",
    "company_name",
    "contact_seniority",
    "company_industry",
    "contact_location_state",
    "contact_location_country",
];

/// True if `family` is one of the provider's known filter families.
pub fn is_known_family(family: &str) -> bool {
    FILTER_FAMILIES.contains(&family)
}

/// `(family, value)` clauses → one Lead Finder filter dict, ANDed across
/// families. Each family gets a single-element `include` list (an OR is strictly
/// dominated). (`discovery.filters_for`)
///
/// Returns `Err` on two clauses of the same family — the one-value-per-family
/// invariant, enforced here because the dict is keyed by family.
pub fn filters_for(clauses: &[(String, String)]) -> Result<Value, String> {
    let mut sorted: Vec<&(String, String)> = clauses.iter().collect();
    sorted.sort();

    let mut seen = BTreeSet::new();
    for (family, _) in &sorted {
        if !seen.insert(family.clone()) {
            let families: Vec<&str> = sorted.iter().map(|(f, _)| f.as_str()).collect();
            return Err(format!(
                "a query holds at most one clause per family, got {families:?}"
            ));
        }
    }

    let mut filters = Map::new();
    for (family, value) in sorted {
        if SCALAR_FAMILIES.contains(&family.as_str()) {
            let n: i64 = value.parse().map_err(|_| {
                format!("scalar family {family} needs an integer value, got {value:?}")
            })?;
            filters.insert(family.clone(), json!(n));
        } else if family == "lead_job_title" {
            filters.insert(
                family.clone(),
                json!({ "include": [value], "exact_match": false }),
            );
        } else {
            filters.insert(family.clone(), json!({ "include": [value] }));
        }\n// revival 2026 touch: src/discovery/mod.rs\n\n// revival 2026 touch: src/discovery/mod.rs\n