//! `core.Task` — the persistent task queue. (`openhaze/core/models.py:Task`)

use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// The four self-scheduling task types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    FindEmail,    // submit leg — fire a paid lookup
    CollectEmail, // poll leg — check an in-flight lookup (payload carries request_id)
    FollowUp,
    Email,
}

impl TaskType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaskType::FindEmail => "find_email",
            TaskType::CollectEmail => "collect_email",
            TaskType::FollowUp => "follow_up",
            TaskType::Email => "email",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "find_email" => TaskType::FindEmail,
            "collect_email" => TaskType::CollectEmail,
            "follow_up" => TaskType::FollowUp,
            "email" => TaskType::Email,
            _ => return None,
        })
    }

    /// Opportunity-cost rank for a single worker: value-to-funnel first.
    /// `follow_up (0) > collect_email (1) > email (2) > find_email (3)`.
    /// Mirrors `TaskQuerySet._priority_order`. Lower is higher priority.
    pub fn priority(self) -> i64 {
        match self {
            TaskType::FollowUp => 0,
            TaskType::CollectEmail => 1,
            TaskType::Email => 2,
            TaskType::FindEmail => 3,
        }
    }
}

/// Task lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {