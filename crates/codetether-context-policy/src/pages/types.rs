//! Page classification and residency contracts.
use super::*;

/// ClawVM page classification for a single chat-history entry.
///
/// Kinds are stable identifiers, not flags — a message has exactly one
/// kind. The degradation path and minimum-fidelity invariant are
/// encoded on [`PageKind`] itself (see
/// [`min_fidelity`](Self::min_fidelity) and
/// [`degradation_path`](Self::degradation_path)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageKind {
    /// System prompts and procedural directives. Losing them causes
    /// "forgot its protocol" failures (ClawVM §2).
    Bootstrap,
    /// Hard-pinned invariants the agent must honour — codetether's
    /// `.tasks.jsonl` goal entries map here natively.
    Constraint,
    /// A formerly pinned source replaced by a later absolute transcript turn.
    /// Its text remains durable evidence but is no longer an active constraint.
    SupersededConstraint { by_turn: usize },
    /// Current objective and step. Live plan state.
    Plan,
    /// Scoped user or project preferences.
    Preference,
    /// Tool outputs. Pointer-safe (Phase B): can degrade to a handle
    /// backed by the MinIO history sink.
    Evidence,
    /// User / assistant conversation spans. The bulk of the transcript.
    Conversation,
}

/// Residency level of a page inside the derived context.
///
/// Multi-resolution representations (ClawVM §3): a page degrades along
/// its [`PageKind::degradation_path`] without ever violating its
/// [`PageKind::min_fidelity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyLevel {
    /// Verbatim original content.
    Full,
    /// Token-reduced text (e.g. LLMLingua-2 compression).
    Compressed,
    /// Typed fields sufficient to satisfy the page's invariant.
    Structured,
    /// Resolvable handle plus minimal metadata. Backed by the MinIO
    /// history sink for Evidence pages.
    Pointer,
}
