//! Native session details needed by import workflows.
use crate::{Restore, State};
use anyhow::Result;
use codetether_context_policy::summary_index::SummaryIndex;
/// Full native JSON decoding and metadata operations for legacy imports.
/// # Implementors
/// Use the same legacy header, summary index and timestamp comparison as native
/// imports. Replace checkpoint state without reusing a potentially poisoned lock.
pub trait Legacy: Restore + serde::de::DeserializeOwned {
    /// Serialize the legacy header without refreshing native provenance.
    /// # Errors
    /// Propagates native header serialization failures.
    fn legacy_header(&self) -> Result<String>;
    /// Borrow the bounded summary cache from the decoded snapshot.
    fn summary_index(&self) -> &SummaryIndex;
    /// Compare whether this snapshot is at least as recent as the incoming one.
    fn is_at_least_as_new_as(&self, incoming: &Self) -> bool;
    /// Install a fresh per-instance checkpoint lock before persistence.
    fn replace_state(&mut self, state: State);
}
