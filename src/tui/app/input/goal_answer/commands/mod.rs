//! Human goal/task commands that never borrow the in-flight session.

mod pending;
mod resume;
mod tasks;
pub(crate) use resume::{drain, restart};

use crate::tui::app::text::{command_with_optional_args, normalize_slash_command};
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};

pub(crate) async fn intercept(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> bool {
    let input = app.state.input.trim().to_string();
    let normalized = normalize_slash_command(&input);
    if let Some(rest) = command_with_optional_args(&normalized, "/tasks") {
        tasks::show(app, slot.view().id(), rest).await;
        app.state.clear_input();
        return true;
    }
    let Some(rest) = command_with_optional_args(&normalized, "/goal") else {
        return false;
    };
    if rest.trim() == "edit" {
        super::editor::open(app, slot.view().id());
        return true;
    }
    let id = slot.view().id();
    let verb = rest.split_whitespace().next().unwrap_or_default();
    let changed = crate::tui::app::commands::goal::handle(app, id, rest).await;
    app.state.push_history(input);
    app.state.clear_input();
    if changed
        && matches!(
            verb,
            "set" | "edit" | "override" | "budget" | "pause" | "resume" | "done" | "clear"
        )
    {
        restart(app, id, runtime);
    }
    app.state.needs_redraw = true;
    true
}

#[cfg(test)]
mod tests;
