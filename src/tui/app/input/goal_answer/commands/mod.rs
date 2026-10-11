//! Human goal/task commands that never borrow the in-flight session.

mod execute;
mod pending;
mod restart_needed;
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
    if command_with_optional_args(&normalized, "/go").is_some() {
        execute::run(app, slot.view().id(), runtime, &normalized).await;
        return true;
    }
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
    if changed && restart_needed::for_verb(verb) {
        restart(app, id, runtime);
    }
    app.state.needs_redraw = true;
    true
}

#[cfg(test)]
mod tests;
