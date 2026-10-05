//! Compose overlays in priority order, with answer satisfaction on top.

use crate::tui::app::state::App;
use ratatui::Frame;

pub(super) fn render(frame: &mut Frame, app: &mut App) {
    if app.state.symbol_search.active {
        crate::tui::symbol_search::render_symbol_search(
            frame,
            &mut app.state.symbol_search,
            frame.area(),
        );
    }
    super::goal_prompt_overlay::render_if_active(frame, frame.area(), &app.state.goal_prompt);
    super::fuzzy_find_overlay::render_if_active(frame, frame.area(), &app.state.fuzzy_find);
    super::interlude::render_if_active(frame, frame.area(), app);
    crate::tui::app::watchdog::render_watchdog_notification(frame, frame.area(), &app.state);
    super::answer_review::render(frame, app);
}
