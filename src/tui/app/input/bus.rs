//! Bus-view input handlers for the TUI.
//!
//! Provides key handlers for scrolling, filtering, and
//! clearing the protocol bus log while in
//! [`ViewMode::Bus`].
//!
//! # Examples
//!
//! ```ignore
//! handle_bus_g(&mut app);
//! handle_bus_c(&mut app);
//! handle_bus_slash(&mut app);
//! ```

use crate::tui::app::state::App;

/// Jump to the bottom of the bus log and enable auto-scroll.
///
/// Triggered by pressing `g` while in Bus view mode.
///
/// # Examples
///
/// ```ignore
/// handle_bus_g(&mut app);
/// ```
pub fn handle_bus_g(app: &mut App) {
    let len = app.state.bus_log.visible_count();
    if len > 0 {
        app.state.bus_log.selected_index = len - 1;
        app.state.bus_log.auto_scroll = true;
    }
}

/// Clear the bus log protocol filter.
///
/// Triggered by pressing `c` while in Bus view mode.
///
/// # Examples
///
/// ```ignore
/// handle_bus_c(&mut app);
/// ```
pub fn handle_bus_c(app: &mut App) {
    app.state.bus_log.clear_filter();
    app.state.status = "Protocol filter cleared".to_string();
}

/// Enter bus filter input mode.
///
/// Triggered by pressing `/` while in Bus view mode.
///
/// # Examples
///
/// ```ignore
/// handle_bus_slash(&mut app);
/// ```
pub fn handle_bus_slash(app: &mut App) {
    app.state.bus_log.enter_filter_mode();
    app.state.status = "Protocol filter mode".to_string();
}

/// Confirm the bus filter and exit filter input mode.
///
/// Called when Enter is pressed while the bus filter input
/// is active.
///
/// # Examples
///
/// ```ignore
/// handle_enter_bus_filter(&mut app);
/// ```
pub(super) fn handle_enter_bus_filter(app: &mut App) {
    app.state.bus_log.exit_filter_mode();
    app.state.status = if app.state.bus_log.filter.is_empty() {
        "Protocol filter cleared".to_string()
    } else {
        format!("Protocol filter applied: {}", app.state.bus_log.filter)
    };
}
