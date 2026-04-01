//! The lattice visit — compose a query, never invent one. (`pipeline/descend.py`)
//!
//! Walk the campaign's clause pool's conjunctions in visit order
//! (level N → N-1 → … → 2 → 1, closest-to-seed first within a level) and hand the
//! frontier the first one not yet fetched and not pruned by the subset test.

use std::collections::{BTreeMap, HashSet};

use anyhow::Result;
use owo_colors::{AnsiColors, OwoColorize};
use sqlx::{Row, SqlitePool};

use crate::discovery::describe_clauses;
use crate::models::{clause_key, DiscoveryQuery, EmptyClauseSet};

/// The campaign's clause pool grouped by family, values in insertion (pk) order —
/// the ICP's own ranking. (`descend._pool`)
pub async fn pool(pool: &SqlitePool, campaign_id: i64) -> Result<BTreeMap<String, Vec<String>>> {
    let rows = sqlx::query(
        "SELECT cl.family, cl.value FROM core_clause cl
         JOIN core_campaign_clauses cc ON cc.clause_id = cl.id
         WHERE cc.campaign_id = ?1 ORDER BY cl.id",
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await?;
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for r in rows {
        map.entry(r.get::<String, _>("family"))
            .or_default()
            .push(r.get::<String, _>("value"));
    }
    Ok(map)
}

/// Every conjunction the pool spans, in visit order. (`descend._visit_order`)
pub fn visit_order(pool: &BTreeMap<String, Vec<String>>) -> Vec<Vec<(String, String)>> {
    let families: Vec<&String> = pool.keys().collect();
    // ranks[family][value] = position in the pool's list.
    let ranks: BTreeMap<&String, BTreeMap<&String, usize>> = families
        .iter()
        .map(|f| {
            let m = pool[*f].iter().enumerate().map(|(i, v)| (v, i)).collect();
            (*f, m)
        })
        .collect();

    // choices per family: each value, plus "unset".
    let choices: Vec<Vec<Option<(String, String)>>> = families
        .iter()
        .map(|f| {
            let mut c: Vec<Option<(String, String)>> =
                pool[*f].iter().map(|v| Some(((*f).clone(), v.clone()))).collect();
            c.push(None);
            c
        })
        .collect();

    // Cartesian product.
    let mut candidates: Vec<Vec<(String, String)>> = vec![Vec::new()];
    for family_choices in &choices {
        let mut next = Vec::new();
        for combo in &candidates {
            for choice in family_choices {
                let mut c = combo.clone();
                if let Some(pair) = choice {
                    c.push(pair.clone());
                }
                next.push(c);
            }
        }
        candidates = next;
    }
    // drop empty, sort each candidate's clauses.
    let mut candidates: Vec<Vec<(String, String)>> = candidates
        .into_iter()
        .filter(|c| !c.is_empty())
        .map(|mut c| {
            c.sort();
            c
        })
        .collect();
    if candidates.is_empty() {
        return candidates;
    }
    let deepest = candidates.iter().map(|c| c.len()).max().unwrap();

    // rank key: (level = deepest - len [deepest first], distance, candidate).
    candidates.sort_by(|a, b| {
        let la = deepest - a.len();
        let lb = deepest - b.len();
        let da: usize = a.iter().map(|(f, v)| ranks[f][v]).sum();
        let db: usize = b.iter().map(|(f, v)| ranks[f][v]).sum();
        (la, da, a).cmp(&(lb, db, b))
    });
    candidates
}

/// Anti-monotone pruning: is `candidate` a superset of a known-empty conjunction?
/// (`descend._is_pruned`)
pub fn is_pruned(candidate: &[(String, String)], empty_sets: &[HashSet<(String, String)>]) -> bool {
    let cand: HashSet<(String, String)> = candidate.iter().cloned().collect();
    empty_sets.iter().any(|empty| empty.is_subset(&cand))
}

/// The next unvisited, unpruned conjunction from the pool, or `[]`. (`descend.descend`)
pub async fn descend(db: &SqlitePool, campaign_id: i64) -> Result<Vec<(String, String)>> {\n// revival 2026 touch: src/pipeline/descend.rs\n
// revival 2026 update: src/pipeline/descend.rs
