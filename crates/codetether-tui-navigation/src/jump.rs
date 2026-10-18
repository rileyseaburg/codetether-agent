//! jump navigation policy.
use crate::{Action, Host};
use codetether_tui_models::ViewMode;

/// Apply home navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_home(app);
/// # }
/// ```
pub fn handle_home(app: &mut impl Host) {
    match app.context().view_mode {
        ViewMode::Chat => app.apply(Action::CursorHome),
        ViewMode::FilePicker => app.apply(Action::FileFirst),
        _ => {}
    }
}

/// Apply end navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_end(app);
/// # }
/// ```
pub fn handle_end(app: &mut impl Host) {
    match app.context().view_mode {
        ViewMode::Chat => app.apply(Action::CursorEnd),
        ViewMode::FilePicker => app.apply(Action::FileLast),
        _ => {}
    }
}
