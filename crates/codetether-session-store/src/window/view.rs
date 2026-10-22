//! Disjoint native field borrows used during working-set eviction.
use crate::State;
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::Message;
use codetether_tracked_vec::TrackedVec;
/// Loaded vectors and checkpoint state for one eviction operation.
/// # Examples
/// ```rust
/// use codetether_session_store::{State, window::{WorkingSet, before_append}};
/// let state = State::default();
/// let mut messages = Default::default();
/// let mut pages = Default::default();
/// let mut tools: codetether_tracked_vec::TrackedVec<()> = Default::default();
/// let mut summaries = Default::default();
/// before_append(WorkingSet { id: "session-123", state: &state,
///     messages: &mut messages, pages: &mut pages,
///     tools: &mut tools, summaries: &mut summaries });
/// assert!(messages.is_empty());
/// ```
pub struct WorkingSet<'a, T> {
    /// Current durable session identity.
    pub id: &'a str,
    /// Instance-local optimistic checkpoint.
    pub state: &'a State,
    /// Loaded messages and their dirty prefix.
    pub messages: &'a mut TrackedVec<Message>,
    /// Classifications aligned with loaded messages.
    pub pages: &'a mut TrackedVec<PageKind>,
    /// Loaded tool-use records.
    pub tools: &'a mut TrackedVec<T>,
    /// Summary index rebased when the loaded message window advances.
    pub summaries: &'a mut SummaryIndex,
}
