//! Explicit human execution, independent of in-flight session ownership.

use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};

pub(super) async fn run(app: &mut App, id: &str, runtime: &TuiSessionHandle, input: &str) {
    let changed = if input == "/go" {
        crate::tui::app::commands::goal::handle(app, id, "execute").await
    } else {
        app.state.status =
            "/go takes no arguments; it executes the current goal without a budget cap".into();
        false
    };
    app.state.push_history(input.to_string());
    app.state.clear_input();
    if changed {
        super::restart(app, id, runtime);
    }
    app.state.needs_redraw = true;
}
