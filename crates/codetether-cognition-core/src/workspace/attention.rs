//! Attention queue contracts for work that needs persona focus.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Source of an attention item.
///
/// Variants: ContestedBelief, FailedCheck, StaleBelief, ProposalTimeout, FailedExecution.
///
/// # Examples
/// ```
/// use codetether_cognition_core::AttentionSource;
/// match AttentionSource::ContestedBelief {
///     AttentionSource::ContestedBelief => {},
///     other => panic!("unexpected value: {other:?}"),
/// }
/// ```
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
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = AttentionItem { id: "a1".into(), topic: "Recheck belief".into(), topic_tags: vec![], priority: 0.8, source_type: AttentionSource::StaleBelief, source_id: "b1".into(), assigned_persona: None, created_at: now, resolved_at: None };
/// assert!(value.resolved_at.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionItem {
    /// Stable record identifier.
    pub id: String,
    /// Subject needing attention.
    pub topic: String,
    /// Tags used to route attention.
    pub topic_tags: Vec<String>,
    /// Relative priority for attention ranking.
    pub priority: f32,
    /// Cause that created the attention item.
    pub source_type: AttentionSource,
    /// Identifier of the originating record.
    pub source_id: String,
    /// Optional persona assigned to resolve the item.
    pub assigned_persona: Option<String>,
    /// Time the record was created.
    pub created_at: DateTime<Utc>,
    /// Resolution timestamp, if resolved.
    pub resolved_at: Option<DateTime<Utc>>,
}
