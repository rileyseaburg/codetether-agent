//! Source-linked evidence retained independently of checkpoint prose.
use serde::{Deserialize, Serialize};

/// Observed lifecycle of a tool call; a returned payload alone proves no success.
/// # Examples
/// ```
/// use codetether_run_checkpoint::ActionStatus;
/// let status = ActionStatus::Returned;
/// assert!(matches!(status, ActionStatus::Returned));
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    /// No result was observed before checkpointing.
    Attempted,
    /// A result arrived without an explicit execution status.
    Returned,
    /// The runtime result explicitly reported success.
    Success,
    /// The runtime result explicitly reported failure.
    Failed,
}

/// A tool attempt matched to its result by provider call ID.
/// # Examples
/// ```
/// use codetether_run_checkpoint::{ActionEvidence, ActionStatus};
/// let action = ActionEvidence { call_id: "c1".into(), tool: "read".into(),
///     source_turn: 4, status: ActionStatus::Attempted, result_turn: None };
/// assert!(action.result_turn.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionEvidence {
    /// Original provider call identifier.
    pub call_id: String,
    /// Requested tool name.
    pub tool: String,
    /// Absolute source message index of the attempt.
    pub source_turn: usize,
    /// Observed outcome, never inferred from assistant prose.
    pub status: ActionStatus,
    /// Absolute result message index, when observed.
    pub result_turn: Option<usize>,
}

/// Complete user text with its absolute transcript reference.
/// # Examples
/// ```
/// let source = codetether_run_checkpoint::UserEvidence {
///     source_turn: 8, text: "Do not deploy".into() };
/// assert_eq!(source.source_turn, 8);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserEvidence {
    /// Absolute source message index.
    pub source_turn: usize,
    /// Complete text, including multiline corrections and constraints.
    pub text: String,
}
