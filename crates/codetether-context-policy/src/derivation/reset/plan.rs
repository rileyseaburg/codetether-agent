//! Prefix, tail and source metadata retained across host summary retrieval.
use super::*;
/// Reset input awaiting a host-provided prefix summary.
///
/// Constructed by [prepare] as [ResetPreparation::Summarize].
pub struct ResetSummaryPlan {
    /// Older messages to summarize.
    pub prefix: Vec<Message>,
    /// Active task messages retained verbatim.
    pub tail: Vec<Message>,
    /// Original canonical message count.
    pub origin_len: usize,
    /// Original index of the retained prefix start.
    pub base_index: usize,
    pub(super) dropped_ranges: Vec<(usize, usize)>,
    pub(super) provenance: Vec<String>,
}
