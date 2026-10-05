//! Record raw input before alias expansion and dispatch.

use crate::tui::app::state::App;

pub(super) fn capture(app: &mut App) -> String {
    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }
    prompt
}
