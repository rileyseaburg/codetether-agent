//! Thought/event type classification for the cognition loop.

use serde::{Deserialize, Serialize};

/// Event types emitted by the cognition loop.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::ThoughtEventType;
/// let kind = ThoughtEventType::ThoughtGenerated;
/// assert_eq!(kind, ThoughtEventType::ThoughtGenerated);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThoughtEventType {
    ThoughtGenerated,
    HypothesisRaised,
    CheckRequested,
    CheckResult,
    ProposalCreated,
    ProposalVerified,
    ProposalRejected,
    ActionExecuted,
    PersonaSpawned,
    PersonaReaped,
    SnapshotCompressed,
    BeliefExtracted,
    BeliefContested,
    BeliefRevalidated,
    BudgetPaused,
    IdleReaped,
    AttentionCreated,
    VoteCast,
    WorkspaceUpdated,
}

#[path = "thought_event_type_label.rs"]
mod label;
