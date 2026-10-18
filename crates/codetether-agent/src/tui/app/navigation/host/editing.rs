//! Native editing operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::HistoryPrevious => {
            let _ = app.state.composer.history_prev();
        }
        Action::HistoryNext => {
            let _ = app.state.composer.history_next();
        }
        Action::CursorWordLeft => {
            app.state.composer.move_cursor_word_left();
        }
        Action::CursorWordRight => {
            app.state.composer.move_cursor_word_right();
        }
        Action::CursorLeft => {
            app.state.composer.move_cursor_left();
        }
        Action::CursorRight => {
            app.state.composer.move_cursor_right();
        }
        Action::DeleteForward => {
            app.state.composer.delete_forward();
        }
        Action::CursorHome => {
            app.state.composer.move_cursor_home();
        }
        Action::CursorEnd => {
            app.state.composer.move_cursor_end();
        }
        _ => return false,
    }
    true
}
