//! Native state bindings for bus monitor controls.
use crate::tui::app::state::App;
pub fn handle_bus_g(app: &mut App) {
    codetether_bus_view::input::handle_bus_g(&mut app.state.bus_log);
}
pub fn handle_bus_c(app: &mut App) {
    codetether_bus_view::input::handle_bus_c(&mut app.state.bus_log, &mut app.state.status);
}
pub fn handle_bus_slash(app: &mut App) {
    codetether_bus_view::input::handle_bus_slash(&mut app.state.bus_log, &mut app.state.status);
}
pub(super) fn handle_enter_bus_filter(app: &mut App) {
    codetether_bus_view::input::handle_enter_bus_filter(
        &mut app.state.bus_log,
        &mut app.state.status,
    );
}
