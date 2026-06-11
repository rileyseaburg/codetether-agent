//! One-shot queued chat prompt used while the main TUI turn is running.

use std::sync::{LazyLock, Mutex};

use crate::tui::app::state::App;

static QUEUED: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

pub fn store(app: &mut App, prompt: String) {
    if prompt.trim().is_empty() {
        return;
    }
    let preview = crate::tui::app::text::truncate_preview(&prompt, 80);
    *QUEUED.lock().expect("queued prompt mutex poisoned") = Some(prompt);
    app.state.clear_input();
    app.state.status = format!("Queued next prompt: {preview}");
}

pub fn take() -> Option<String> {
    QUEUED.lock().expect("queued prompt mutex poisoned").take()
}

pub fn has_pending() -> bool {
    QUEUED
        .lock()
        .expect("queued prompt mutex poisoned")
        .is_some()
}
