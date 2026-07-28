//! Page-local clipboard storage for agent actions.

use crate::browser_agent::page::BrowserPage;

const CLIPBOARD_KEY: &str = "__browser_agent_clipboard_text";

impl BrowserPage {
    /// Read the deterministic page clipboard text.
    pub fn read_clipboard(&self) -> String {
        self.session
            .storage
            .get(CLIPBOARD_KEY)
            .cloned()
            .unwrap_or_default()
    }

    /// Replace the deterministic page clipboard text.
    pub fn write_clipboard(&mut self, text: impl Into<String>) {
        self.session
            .storage
            .insert(CLIPBOARD_KEY.into(), text.into());
    }

    /// Clear the deterministic page clipboard text.
    pub fn clear_clipboard(&mut self) {
        self.session.storage.remove(CLIPBOARD_KEY);
    }
}
