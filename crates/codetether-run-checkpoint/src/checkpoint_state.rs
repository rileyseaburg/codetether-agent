//! State extraction from session messages for checkpoint construction.

/// Intermediate state extracted from session messages.
#[derive(Debug, Default)]
pub(super) struct ExtractedState {
    pub(super) actions: Vec<crate::ActionEvidence>,
    pub(super) user_context: Vec<crate::UserEvidence>,
    pub(super) assistant_context: Option<String>,
    pub(super) browser_url: Option<String>,
    pub(super) completed_actions: Vec<String>,
    pub(super) blockers: Vec<String>,
    pub(super) next_action: String,
}

/// Extract checkpoint-relevant state from the session transcript.
pub(super) fn extract(
    messages: &[codetether_provider_core::Message],
    offset: usize,
) -> ExtractedState {
    let mut state = super::checkpoint_walk::scan(messages, offset);
    state.next_action = state.next_action.or_else(default_next_action);
    state.into_extracted()
}

fn default_next_action() -> Option<String> {
    Some("Continue from the last assistant/tool state toward the original objective.".into())
}
