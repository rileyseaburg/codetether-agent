//! Application redraw and activity-clock adapters for question input.
use crate::tui::app::state::App;
#[cfg(test)]
use codetether_question::runtime;
use crossterm::event::KeyEvent;
#[cfg(test)]
use crossterm::event::{KeyCode, KeyModifiers};
pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, key: KeyEvent) -> bool {
    let id = app.state.session_id.as_deref();
    let redraw = &mut app.state.needs_redraw;
    let last_event = &mut app.state.main_last_event_at;
    codetether_question_view::handle_key(
        id,
        key,
        || *redraw = true,
        || *last_event = Some(std::time::Instant::now()),
    )
}
pub(super) fn paste(app: &mut App, text: &str) -> bool {
    let handled = codetether_question_view::paste(app.state.session_id.as_deref(), text);
    app.state.needs_redraw |= handled;
    handled
}
#[cfg(test)]
#[path = "question_key_tests.rs"]
mod tests;
