//! One-shot queued chat prompt used while the main TUI turn is running.

use crate::tui::app::state::App;

pub fn store(app: &mut App, prompt: String) {
    if prompt.trim().is_empty() {
        return;
    }
    let preview = crate::tui::app::text::truncate_preview(&prompt, 80);
    codetether_session_steering::follow_up::store(prompt);
    app.state.clear_input();
    app.state.status = format!("Queued next prompt: {preview}");
}
pub use codetether_session_steering::follow_up::has_pending;
pub use codetether_session_steering::follow_up::take;
