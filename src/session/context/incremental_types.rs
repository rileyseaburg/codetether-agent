//! Shared types for incremental derivation helpers.

use crate::session::index::SummaryRange;

/// Produced summary for a dropped transcript gap.
#[derive(Debug, Clone)]
pub struct SummaryGap {
    /// Original transcript range covered by the summary.
    pub range: SummaryRange,
    /// Budget-bounded summary content.
    pub content: String,
}

/// What a message in the derived projection came from. Tracked
/// parallel to the message vector so `dropped_ranges` can be
/// recomputed after pairing repair and overflow clamping (issue #231
/// item 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageOrigin {
    /// Original transcript message at index `usize` in the pre-derivation clone.
    Clone(usize),
    /// Summary covering `range` in the pre-derivation clone.
    Summary(SummaryRange),
    /// Synthetic placeholder injected by pairing repair.
    Synthetic,
}
