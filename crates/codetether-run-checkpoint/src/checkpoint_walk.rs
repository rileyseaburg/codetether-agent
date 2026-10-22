//! Transcript walker for checkpoint state extraction.

use super::{checkpoint_assistant, checkpoint_state::ExtractedState, checkpoint_tool};
use codetether_provider_core::{Message, Role};

#[derive(Default)]
pub(super) struct ScanState {
    pub(super) actions: Vec<crate::ActionEvidence>,
    pub(super) user_context: Vec<crate::UserEvidence>,
    pub(super) assistant_context: Option<String>,
    pub(super) source_turn: usize,
    pub(super) browser_url: Option<String>,
    pub(super) completed_actions: Vec<String>,
    pub(super) blockers: Vec<String>,
    pub(super) next_action: Option<String>,
}

pub(super) fn scan(messages: &[Message], offset: usize) -> ScanState {
    let mut state = ScanState::default();
    for (index, msg) in messages.iter().enumerate() {
        state.source_turn = offset.saturating_add(index);
        scan_message(msg, &mut state);
    }
    state
}

impl ScanState {
    pub(super) fn into_extracted(self) -> ExtractedState {
        ExtractedState {
            actions: self.actions,
            user_context: self.user_context,
            assistant_context: self.assistant_context,
            browser_url: self.browser_url,
            completed_actions: self.completed_actions,
            blockers: self.blockers,
            next_action: self.next_action.unwrap_or_default(),
        }
    }
}

pub(super) fn scan_message(msg: &Message, state: &mut ScanState) {
    match msg.role {
        Role::Tool => checkpoint_tool::scan(&msg.content, state),
        Role::Assistant => checkpoint_assistant::scan(&msg.content, state),
        Role::User => super::checkpoint_user::scan(&msg.content, state),
        Role::System | Role::Developer => {}
    }
}
