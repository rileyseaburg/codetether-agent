//! Serialized session task-log events.

use super::{GoalRuntimeUpdate, GoalSourceKind, SessionTaskStatus};
use chrono::{DateTime, Utc};
/// A single durable event in the session task log.
///
/// `TaskEvent` is the append-only record format for session goal governance and
/// task lifecycle tracking. Each variant captures one fact that happened at a
/// specific UTC timestamp. The current materialized task state is reconstructed
/// by replaying these events in order, rather than by editing earlier records.
///
/// Events are serialized with an internally tagged Serde representation. The
/// `kind` field stores the variant name in `snake_case`, which keeps persisted
/// JSONL task logs readable and stable across releases.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TaskEvent {
    /// Declares or replaces the active session goal.
    ///
    /// A goal contains the objective the agent should pursue, optional
    /// completion criteria, explicit forbidden behaviors, and provenance fields
    /// describing where the objective came from. When this event is folded into
    /// task state, it supersedes any previously active goal.
    GoalSet {
        /// Stable identifier for accounting and stale-update rejection.
        #[serde(default)]
        goal_id: String,
        /// Time at which the goal was declared.
        at: DateTime<Utc>,
        /// Human-readable objective the agent should work toward.
        objective: String,
        /// Criteria that indicate the objective has been satisfied.
        ///
        /// Defaults to an empty list when omitted by older log entries or simple
        /// goal-setting paths.
        #[serde(default)]
        success_criteria: Vec<String>,
        /// Actions or behaviors that are explicitly out of scope.
        ///
        /// Defaults to an empty list when no additional forbidden behaviors were
        /// recorded.
        #[serde(default)]
        forbidden: Vec<String>,
        /// Identifier of the session containing the source evidence for this
        /// goal.
        ///
        /// Empty when the producer did not capture a source session.
        #[serde(default)]
        source_session_id: String,
        /// Turn identifier or turn index within the source session.
        ///
        /// Empty when the goal is not anchored to a specific raw transcript
        /// turn.
        #[serde(default)]
        source_turn_id: String,
        /// Hash of the source text used to derive the goal.
        ///
        /// This allows provenance comparisons without duplicating the full
        /// source text in every event.
        #[serde(default)]
        source_text_hash: String,
        /// Category describing how the goal was derived.
        ///
        /// Consumers can use this to distinguish strong provenance, such as raw
        /// transcript turns or direct user input, from weaker provenance, such
        /// as summarized recall, durable memory, or inference.
        #[serde(default)]
        source_kind: GoalSourceKind,
        /// Producer-supplied confidence in the recorded provenance.
        ///
        /// `1.0` means the goal is directly supported by the recorded source
        /// evidence. `0.0` means no confidence was supplied or the goal should be
        /// treated as unverified.
        #[serde(default)]
        confidence: f32,
    },
    /// Applies a lifecycle, accounting, or continuation update to the goal.
    GoalRuntime { #[serde(flatten)] update: GoalRuntimeUpdate },
    /// Records that the agent reaffirmed alignment with the active goal.
    ///
    /// Reaffirmation does not change the objective. It stores a progress note so
    /// governance logic can reset drift counters and later explain why work
    /// continued.
    GoalReaffirmed {
        /// Time at which the reaffirmation was recorded.
        at: DateTime<Utc>,
        /// Concise statement of completed work, remaining work, or the current
        /// blocker.
        progress_note: String,
    },
    /// Clears the active session goal.
    ///
    /// Folding the log after this event leaves the session without an active
    /// goal until a later [`TaskEvent::GoalSet`] appears.
    GoalCleared { at: DateTime<Utc>, reason: String },
    /// Adds a task to the session task list.
    ///
    /// Added tasks start as pending in folded state. Later
    /// [`TaskEvent::TaskStatus`] events update tasks by matching the `id` field.
    TaskAdded {
        /// Time at which the task was added.
        at: DateTime<Utc>,
        /// Stable identifier for this task within the session task log.
        id: String,
        /// Human-readable task description.
        content: String,
        /// Optional parent task identifier for nested task breakdowns.
        ///
        /// Defaults to `None` for top-level tasks and older log entries.
        #[serde(default)]
        parent_id: Option<String>,
    },
    /// Updates the lifecycle status of an existing task.
    ///
    /// Folding logic applies this transition to the task with the matching `id`
    /// when that task exists in the materialized task map. The event remains in
    /// the append-only log even if the task is missing.
    TaskStatus {
        /// Time at which the status transition was recorded.
        at: DateTime<Utc>,
        /// Identifier of the task being updated.
        id: String,
        /// New lifecycle status for the task.
        status: SessionTaskStatus,
        /// Optional note explaining the transition, such as a blocker,
        /// cancellation reason, or completion summary.
        #[serde(default)]
        note: Option<String>,
    },
    /// Records that governance detected possible drift from the active goal.
    ///
    /// Drift events preserve the counters that triggered an alignment warning,
    /// allowing later inspection to understand why the agent was asked to
    /// reaffirm the goal or stop before continuing.
    DriftDetected {
        /// Time at which drift was detected.
        at: DateTime<Utc>,
        /// Number of tool calls since the last goal reaffirmation.
        tool_calls_since_reaffirm: u32,
        /// Number of errors since the last goal reaffirmation.
        errors_since_reaffirm: u32,
    },
}
