//! State extraction from session messages for checkpoint construction.

/// Intermediate state extracted from session messages.
#[derive(Debug, Default)]
pub(super) struct ExtractedState {
    pub(super) browser_url: Option<String>,
    pub(super) completed_actions: Vec<String>,
    pub(super) blockers: Vec<String>,
    pub(super) next_action: String,
}

/// Extract checkpoint-relevant state from the session transcript.
pub(super) fn extract(messages: &[crate::provider::Message]) -> ExtractedState {
    let mut state = super::checkpoint_walk::scan(messages);
    state.next_action = state.next_action.or_else(default_next_action);
    state.into_extracted()
}

fn default_next_action() -> Option<String> {
    Some("Continue from the last assistant/tool state toward the original objective.".into())
}
