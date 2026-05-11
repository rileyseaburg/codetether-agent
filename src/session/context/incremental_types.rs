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
