//! Overlay scroll checks for mouse wheel events.
//!
//! Before the view-mode match, mouse scroll events need to
//! check whether a floating overlay (symbol search, session
//! list, model picker, slash suggestions) should consume the
//! scroll instead.
//!
//! # Examples
//!
//! ```ignore
//! if scroll_overlay_up(&mut app) { return; }
//! ```

use crate::tui::app::state::App;
use crate::tui::app::symbols::symbol_search_active;
use crate::tui::models::ViewMode;

/// Try to scroll an overlay upward.
///
/// Returns `true` if an overlay consumed the event, meaning
/// the caller should skip the view-mode scroll dispatch.
///
/// # Examples
///
/// ```ignore
/// if scroll_overlay_up(&mut app) { return; }
/// ```
pub(super) fn scroll_overlay_up(app: &mut App, amount: usize) -> bool {
    if symbol_search_active(app) {
        for _ in 0..amount {
            app.state.symbol_search.select_prev();
        }
        return true;
    }
    if app.state.view_mode == ViewMode::Sessions {
        for _ in 0..amount {
            app.state.sessions_select_prev();
        }
        return true;
    }
    if app.state.view_mode == ViewMode::Model {
        for _ in 0..amount {
            app.state.model_select_prev();
        }
        return true;
    }
    if app.state.slash_suggestions_navigable() {
        for _ in 0..amount {
            app.state.select_prev_slash_suggestion();
        }
        return true;
    }
    false
}

/// Try to scroll an overlay downward.
///
/// Mirror of [`scroll_overlay_up`] for the down direction.
///
/// # Examples
///
/// ```ignore
/// if scroll_overlay_down(&mut app) { return; }
/// ```
pub(super) fn scroll_overlay_down(app: &mut App, amount: usize) -> bool {
    if symbol_search_active(app) {
        for _ in 0..amount {
            app.state.symbol_search.select_next();
        }
        return true;
    }
    if app.state.view_mode == ViewMode::Sessions {
        for _ in 0..amount {
            app.state.sessions_select_next();
        }
        return true;
    }
    if app.state.view_mode == ViewMode::Model {
        for _ in 0..amount {
            app.state.model_select_next();
        }
        return true;
    }
    if app.state.slash_suggestions_navigable() {
        for _ in 0..amount {
            app.state.select_next_slash_suggestion();
        }
        return true;
    }
    false
}
