//! Shared helper for `!command` background-shell tests.

use crate::tui::app::input::shell_bg::drain_shell_events;
use crate::tui::app::state::App;

/// Run a `!command` and block until the background task delivers its result
/// into the transcript via the event-loop drain path.
pub(super) async fn run_and_drain(app: &mut App, cwd: &std::path::Path, prompt: &str) -> bool {
    let consumed = super::run(app, cwd, prompt).await;
    if consumed && app.state.shell_running {
        for _ in 0..600 {
            if drain_shell_events(app) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
    consumed
}
