//! Outputs accumulated while processing a cognition tick.
use crate::{MemorySnapshot, Proposal, ThoughtEvent};

/// Products accumulated from thinking work items during a tick.
///
/// # Examples
/// ```
/// use codetether_cognition_core::thought::TickOutput;
/// let output = TickOutput::default();
/// assert!(output.events.is_empty());
/// assert!(output.snapshots.is_empty());
/// assert!(output.proposals.is_empty());
/// ```
#[derive(Default)]
pub struct TickOutput {
    /// Thought and governance events awaiting publication.
    pub events: Vec<ThoughtEvent>,
    /// Memory snapshots awaiting storage.
    pub snapshots: Vec<MemorySnapshot>,
    /// New proposals awaiting insertion.
    pub proposals: Vec<Proposal>,
}
