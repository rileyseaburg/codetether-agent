//! Bus monitor selection and filter controls.
use crate::BusLogState;

/// Jump to the bottom of the bus log and enable auto-scroll.
///
/// Triggered by pressing `g` while in Bus view mode.
///
/// # Examples
///
/// ```
/// let mut state = codetether_bus_view::BusLogState::new();
/// codetether_bus_view::input::handle_bus_g(&mut state);
/// ```
/// # Arguments
/// Receives the bus-log state.
/// # Returns
/// Applies the existing selection or filter transition.
pub fn handle_bus_g(state: &mut BusLogState) {
    let len = state.visible_count();
    if len > 0 {
        state.selected_index = len - 1;
        state.auto_scroll = true;
    }
}

/// Clear the bus log protocol filter.
///
/// Triggered by pressing `c` while in Bus view mode.
///
/// # Examples
///
/// ```
/// let mut state = codetether_bus_view::BusLogState::new();
/// let mut status = String::new();
/// codetether_bus_view::input::handle_bus_c(&mut state, &mut status);
/// ```
/// # Arguments
/// Receives the bus-log state and frontend status text.
/// # Returns
/// Applies the existing selection or filter transition.
pub fn handle_bus_c(state: &mut BusLogState, status: &mut String) {
    state.clear_filter();
    *status = "Protocol filter cleared".to_string();
}

/// Enter bus filter input mode.
///
/// Triggered by pressing `/` while in Bus view mode.
///
/// # Examples
///
/// ```
/// let mut state = codetether_bus_view::BusLogState::new();
/// let mut status = String::new();
/// codetether_bus_view::input::handle_bus_slash(&mut state, &mut status);
/// ```
/// # Arguments
/// Receives the bus-log state and frontend status text.
/// # Returns
/// Applies the existing selection or filter transition.
pub fn handle_bus_slash(state: &mut BusLogState, status: &mut String) {
    state.enter_filter_mode();
    *status = "Protocol filter mode".to_string();
}

/// Confirm the bus filter and exit filter input mode.
///
/// Called when Enter is pressed while the bus filter input
/// is active.
///
/// # Examples
///
/// ```
/// let mut state = codetether_bus_view::BusLogState::new();
/// let mut status = String::new();
/// codetether_bus_view::input::handle_enter_bus_filter(&mut state, &mut status);
/// ```
/// # Arguments
/// Receives the bus-log state and frontend status text.
/// # Returns
/// Applies the existing selection or filter transition.
pub fn handle_enter_bus_filter(state: &mut BusLogState, status: &mut String) {
    state.exit_filter_mode();
    *status = if state.filter.is_empty() {
        "Protocol filter cleared".to_string()
    } else {
        format!("Protocol filter applied: {}", state.filter)
    };
}
