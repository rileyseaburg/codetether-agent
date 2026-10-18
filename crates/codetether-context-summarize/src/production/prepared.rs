//! Owned inputs captured before borrowing the mutable summary cache.
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;
use codetether_tracked_vec::TrackedVec;
use std::sync::Arc;
/// Session inputs retained for one on-demand summary.
/// # Examples
/// ```
/// let _prepared = codetether_context_summarize::production::Prepared::new(
///     Default::default(), 0, Default::default(), "session".into(), None, None,
/// );
/// ```
pub struct Prepared {
    pub(super) messages: TrackedVec<Message>,
    pub(super) generation: u64,
    pub(super) rlm: RlmConfig,
    pub(super) session_id: String,
    pub(super) sub_provider: Option<Arc<dyn Provider>>,
    pub(super) sub_model: Option<String>,
}
impl Prepared {
    /// Retain the captured transcript, cache generation, RLM config and subcall inputs.
    /// # Arguments
    /// Values are moved from the application's existing snapshot without cloning.
    /// # Returns
    /// Prepared inputs for [`super::produce`].
    /// # Examples
    /// See [`Prepared`] for construction.
    pub fn new(
        messages: TrackedVec<Message>,
        generation: u64,
        rlm: RlmConfig,
        session_id: String,
        sub_provider: Option<Arc<dyn Provider>>,
        sub_model: Option<String>,
    ) -> Self {
        Self {
            messages,
            generation,
            rlm,
            session_id,
            sub_provider,
            sub_model,
        }
    }
}
