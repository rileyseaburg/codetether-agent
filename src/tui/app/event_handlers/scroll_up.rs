//! View-mode-aware scroll-up dispatch for mouse wheel events.
//!
//! Routes upward scroll actions to the correct pane based on
//! the active [`ViewMode`] — chat offset, swarm/ralph list,
//! or bus log.
//!
//! # Examples
//!
//! ```ignore
//! scroll_up_by_mode(&mut app, 3);
//! ```

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Route a scroll-up action to the active view pane.
///
/// Dispatches based on [`ViewMode`], scrolling the chat
/// offset, swarm/ralph/bus list, or detail pane.  Modes
/// without scrollable content are ignored.
///
/// # Examples
///
/// ```ignore
/// scroll_up_by_mode(&mut app, 3);
/// ```
pub(super) fn scroll_up_by_mode(app: &mut App, amount: usize) {
    match app.state.view_mode {
        ViewMode::Chat => app.state.scroll_up(amount),
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state.swarm.detail_scroll_up(amount);
            } else {
                for _ in 0..amount {
                    app.state.swarm.select_prev();
                }
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state.ralph.detail_scroll_up(amount);
            } else {
                for _ in 0..amount {
                    app.state.ralph.select_prev();
                }
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state.bus_log.detail_scroll_up(amount);
            } else {
                for _ in 0..amount {
                    app.state.bus_log.select_prev();
                }
            }
        }
        _ => {}
    }
}
