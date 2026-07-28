//! Page helpers for synchronizing browser context state.

use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub(crate) fn attach_context_state(&mut self, state: SharedContextState, adopt: bool) {
        if adopt {
            state.borrow_mut().absorb_session(&self.session);
        } else {
            state.borrow().apply_to_session(&mut self.session);
        }
        self.context_state = Some(state);
        self.runtime = None;
    }

    pub(crate) fn sync_context_state_into_session(&mut self) {
        if let Some(state) = &self.context_state {
            state.borrow().apply_to_session(&mut self.session);
        }
    }

    pub(crate) fn sync_context_state_from_session(&self) {
        if let Some(state) = &self.context_state {
            state.borrow_mut().absorb_session(&self.session);
        }
    }
}
