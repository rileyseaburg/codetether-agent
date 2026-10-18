//! Native entry point for shell-prefixed composer input.
use crate::tui::app::state::App;
use std::path::Path;
/// Consume shell input through the shared background command lifecycle.
pub(in crate::tui::app::input) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
    codetether_chat_shell::submit(app, cwd, prompt)
}
