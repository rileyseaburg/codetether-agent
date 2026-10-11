//! Goal provenance categories.
//!
//! This module defines the trust category attached to a session goal when it is
//! written to the append-only task log. The category records where the goal came
//! from, allowing recovery, governance, and debugging code to distinguish goals
//! grounded in raw transcript evidence from goals inferred through weaker or
//! lossy sources.
//!
//! The enum is serialized as `snake_case` so goal provenance remains stable and
//! readable in persisted JSONL task logs.

use serde::{Deserialize, Serialize};

/// Provenance category for a declared session goal.
///
/// `GoalSourceKind` describes the evidence source used to create a session
/// objective. Consumers should treat variants as a rough trust hierarchy:
/// direct raw-turn or user-provided goals are stronger than summarized recall,
/// memory-derived, or inferred goals.
///
/// The default is [`GoalSourceKind::Inferred`], which is intentionally the
/// weakest category. This keeps older log entries and omitted provenance fields
/// safe by marking them as unverified rather than pretending they came from a
/// canonical user turn.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalSourceKind {
    /// The goal was anchored to a raw transcript turn.
    ///
    /// Use this when the objective is derived directly from canonical session
    /// history, such as a specific user message returned by `context_browse`.
    RawTurn,

    /// The goal was explicitly provided by the user.
    ///
    /// This is appropriate for direct goal-setting interfaces such as a TUI
    /// `/goal set ...` command or a user request that intentionally declares the
    /// current objective.
    UserProvided,

    /// The goal was derived from summarized session recall.
    ///
    /// Recall summaries are useful for recovery but may omit, blend, or
    /// mis-rank details from raw history. Consumers should verify this kind
    /// against transcript turns before treating it as authoritative.
    RecallSummary,

    /// The goal was derived from curated durable memory.
    ///
    /// Memory can preserve important decisions across sessions, but it may be
    /// stale or broader than the current conversation. Consumers should consider
    /// memory scope and timestamp before using it as a hard constraint.
    Memory,

    /// The goal was inferred without stronger provenance.
    ///
    /// This is the fallback for missing provenance and should be treated as
    /// low-confidence until confirmed by the user or anchored to raw transcript
    /// evidence.
    #[default]
    Inferred,
}
