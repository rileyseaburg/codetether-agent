//! Present shared mux command results in the TUI.
mod output;
use crate::tui::app::state::App;
use std::path::Path;
/// Handle mux commands before a chat session is borrowed.
pub(super) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
    let Some(result) = codetether_mux_command::run(cwd, prompt).await else {
        return false;
    };
    output::show(app, result);
    app.state.clear_input();
    true
}
