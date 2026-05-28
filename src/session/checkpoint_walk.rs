//! Transcript walker for checkpoint state extraction.

use super::{checkpoint_assistant, checkpoint_state::ExtractedState, checkpoint_tool};
use crate::provider::{Message, Role};

#[derive(Default)]
pub(super) struct ScanState {
    pub(super) browser_url: Option<String>,
    pub(super) completed_actions: Vec<String>,
    pub(super) blockers: Vec<String>,
    pub(super) next_action: Option<String>,
}

pub(super) fn scan(messages: &[Message]) -> ScanState {
    let mut state = ScanState::default();
    for msg in messages {
        scan_message(msg, &mut state);
    }
    state
}

impl ScanState {
    pub(super) fn into_extracted(self) -> ExtractedState {
        ExtractedState {
            browser_url: self.browser_url,
            completed_actions: self.completed_actions,
            blockers: self.blockers,
            next_action: self.next_action.unwrap_or_default(),
        }
    }
}

fn scan_message(msg: &Message, state: &mut ScanState) {
    match msg.role {
        Role::Tool => checkpoint_tool::scan(&msg.content, state),
        Role::Assistant => checkpoint_assistant::scan(&msg.content, state),
        Role::User | Role::System => {}
    }
}
