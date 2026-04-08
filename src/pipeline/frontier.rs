//! Discovery frontier — a lazy best-first walk over a campaign's query nodes.
//! (`pipeline/frontier.py`)
//!
//! `next_query` picks exactly one query to fetch: **deepen** a node that has