//! Task-queue worker — a port of `core/daemon.py`.
//!
//! Single-threaded: one task at a time. Startup + idle `reconcile`, an
//! active-hours guard, `claim_next` in opportunity-cost order, dispatch via the
//! handler map, then sleep to the next due task. Freemium (kit) campaigns are out
//! of scope for this port.
\n// revival 2026 touch: src/daemon.rs\n