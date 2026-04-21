//! View-mode-aware scroll-down dispatch for mouse wheel events.
//!
//! Routes downward scroll actions to the correct pane based on
//! the active [`ViewMode`] — chat offset, swarm/ralph list,
//! or bus log.
//!
//! # Examples
//!
//! ```ignore
//! scroll_down_by_mode(&mut app, 3);
//! ```

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Route a scroll-down action to the active view pane.
///
/// Mirror of [`super::scroll_up::scroll_up_by_mode`] for the
/// downward direction.  Dispatches based on [`ViewMode`].
///
/// # Examples
///
/// ```ignore
/// scroll_down_by_mode(&mut app, 3);
/// ```
pub(super) fn scroll_down_by_mode(app: &mut App, amount: usize) {
    match app.state.view_mode {
        ViewMode::Chat => app.state.scroll_down(amount),
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state.swarm.detail_scroll_down(amount);
            } else {
                for _ in 0..amount {
                    app.state.swarm.select_next();
                }
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state.ralph.detail_scroll_down(amount);
            } else {
                for _ in 0..amount {
                    app.state.ralph.select_next();
                }
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state.bus_log.detail_scroll_down(amount);
            } else {
                for _ in 0..amount {
                    app.state.bus_log.select_next();
                }
            }
        }
        ViewMode::Audit => {
            for _ in 0..amount {
                app.state.audit.select_next();
            }
        }
        _ => {}
    }
}
