//! Spawn a `!command` on a detached task and stream the result back.

use std::path::Path;

use tokio::sync::mpsc;

use super::run::run;
use crate::tui::app::state::App;

/// Spawn `command` in `cwd` on a background task. The result is delivered via
/// `app.state.shell_rx`, drained by the event loop. Returns immediately.
pub(crate) fn spawn_shell_command(app: &mut App, cwd: &Path, command: String) {
    let (tx, rx) = mpsc::unbounded_channel();
    app.state.shell_rx = Some(rx);
    app.state.shell_running = true;
    let cwd = cwd.to_path_buf();
    tokio::spawn(async move {
        let event = run(command, &cwd).await;
        let _ = tx.send(event);
    });
}
