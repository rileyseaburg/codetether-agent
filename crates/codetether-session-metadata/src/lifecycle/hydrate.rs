use crate::SessionMetadata;
use codetether_context_policy::pages::{PageKind, classify_all};
use codetether_provider_core::Message;
use codetether_tracked_vec::TrackedVec;
/// Restore runtime sidecars after loading persisted session records.
///
/// # Arguments
/// * `metadata` — Session settings and optional execution provenance.
/// * `messages` — Loaded transcript used to rebuild mismatched page sidecars.
/// * `pages` — Mutation-tracked classifications aligned with the transcript.
/// # Returns
/// Repairs mismatched classifications, restores only missing history-sink
/// settings and attaches the current runtime identity to existing provenance.
/// # Examples
/// ```
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let mut pages = Default::default();
/// codetether_session_metadata::lifecycle::hydrate(&mut metadata, &[], &mut pages);
/// assert!(pages.is_empty());
/// ```
pub fn hydrate(
    metadata: &mut SessionMetadata,
    messages: &[Message],
    pages: &mut TrackedVec<PageKind>,
) {
    if pages.len() != messages.len() {
        *pages = classify_all(messages).into();
    }
    if metadata.history_sink.is_none() {
        metadata.history_sink = codetether_history_sink::HistorySinkConfig::from_env()
            .ok()
            .flatten();
    }
    if let Some(identity) = codetether_provenance::runtime_agent_identity()
        && let Some(provenance) = metadata.provenance.as_mut()
    {
        provenance.identity.agent_identity_id = Some(identity);
    }
}
