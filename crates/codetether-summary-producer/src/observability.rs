//! Optional bus + trace_id handle for [`super::produce_summary`].

use uuid::Uuid;

/// Optional observation metadata retained for caller compatibility.
/// The current producer accepts these values without emitting events.
/// # Examples
/// ```
/// let metadata = codetether_summary_producer::SummaryObservability::<()>::default();
/// assert!(metadata.bus.is_none());
/// assert!(metadata.trace_id.is_none());
/// ```
pub struct SummaryObservability<B> {
    /// Optional host-specific event bus.
    pub bus: Option<B>,
    /// Optional parent trace identifier.
    pub trace_id: Option<Uuid>,
}

impl<B> Default for SummaryObservability<B> {
    fn default() -> Self {
        Self {
            bus: None,
            trace_id: None,
        }
    }
}
