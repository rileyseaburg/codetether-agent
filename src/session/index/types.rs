//! Core types for the hierarchical summary index.
//!
//! These types are persisted as part of the session JSON and must
//! remain serde-compatible across restarts.

use serde::{Deserialize, Serialize};

/// Half-open `[start, end)` range over session message indices.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::index::SummaryRange;
///
/// let r = SummaryRange::new(2, 5).unwrap();
/// assert!(r.contains(2));
/// assert!(r.contains(4));
/// assert!(!r.contains(5));
/// assert!(SummaryRange::new(5, 5).is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SummaryRange {
    /// Inclusive lower bound (message index in session transcript).
    pub start: usize,
    /// Exclusive upper bound.
    pub end: usize,
}

impl SummaryRange {
    /// Construct a range. Returns `None` if `start >= end`.
    pub fn new(start: usize, end: usize) -> Option<Self> {
        (start < end).then_some(Self { start, end })
    }

    /// Whether this range covers message index `idx`.
    pub fn contains(&self, idx: usize) -> bool {
        self.start <= idx && idx < self.end
    }
}

/// Granularity at which a [`SummaryNode`] was produced.
///
/// Step 18 uses this to choose between turn-level RLM summaries
/// and coarser phase-/session-level rollups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Granularity {
    /// Single turn or small window of turns.
    Turn,
    /// A logical phase of work (e.g. "implement feature X").
    Phase,
    /// Full session summarisation.
    Session,
}

/// One cached summary entry — produced once, consumed many times until
/// the underlying range is invalidated.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::index::{Granularity, SummaryNode};
///
/// let node = SummaryNode {
///     content: "Discussed auth refactor".into(),
///     target_tokens: 512,
///     granularity: Granularity::Turn,
///     generation: 1,
/// };
/// assert_eq!(node.content, "Discussed auth refactor");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryNode {
    /// The summary text.
    pub content: String,
    /// Target token budget the producer was asked to fit.
    pub target_tokens: usize,
    /// Granularity tag.
    pub granularity: Granularity,
    /// Generation counter when this node was produced.
    pub generation: u64,
}

/// Upper bound on cached summaries before LRU eviction kicks in.
///
/// Chosen so the index sidecar stays under ~1 MB for typical sessions.
pub const MAX_CACHED_SUMMARIES: usize = 128;
