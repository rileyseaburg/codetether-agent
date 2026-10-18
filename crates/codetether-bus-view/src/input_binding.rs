//! Frontend bus-log and status fields for monitor key actions.
/// Declare bus monitor controls over the existing frontend state.
/// # Examples
/// ```text
/// codetether_bus_view::declare_frontend_bus_input!(App);
/// ```
#[macro_export]
macro_rules! declare_frontend_bus_input {
    ($app:path $(,)?) => {
        use $app as App;
        pub fn handle_bus_g(app: &mut App) {
            $crate::input::handle_bus_g(&mut app.state.bus_log);
        }
        pub fn handle_bus_c(app: &mut App) {
            $crate::input::handle_bus_c(&mut app.state.bus_log, &mut app.state.status);
        }
        pub fn handle_bus_slash(app: &mut App) {
            $crate::input::handle_bus_slash(&mut app.state.bus_log, &mut app.state.status);
        }
        pub(super) fn handle_enter_bus_filter(app: &mut App) {
            $crate::input::handle_enter_bus_filter(&mut app.state.bus_log, &mut app.state.status);
        }
    };
}
