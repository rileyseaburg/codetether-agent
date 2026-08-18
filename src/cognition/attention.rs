//! Attention queue contracts for work that needs persona focus.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Source of an attention item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSource {
    ContestedBelief,
    FailedCheck,
    StaleBelief,
    ProposalTimeout,
    FailedExecution,
}

/// An item requiring persona attention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionItem {
    pub id: String,
    pub topic: String,
    pub topic_tags: Vec<String>,
    pub priority: f32,
    pub source_type: AttentionSource,
    pub source_id: String,
    pub assigned_persona: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}
