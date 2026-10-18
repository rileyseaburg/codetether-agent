//! Frontend text buffers, filters and symbols for character/backspace input.
mod buffer;
mod filters;
mod symbols;
/// Declare frontend text effects and public character/backspace entry points.
///
/// The shared policy retains input routing; the selected callbacks implement
/// file filtering, mention controls and symbol refresh with frontend state.
/// # Examples
/// See the frontend text-host declaration for concrete types and callbacks.
#[macro_export]
macro_rules! declare_frontend_text_input {
    ($app:path, $mode:path, $input:path, $modifiers:path;
     $active:path, $push:path, $backspace:path, $dismiss:path, $open:path, $refresh:path $(,)?) => {
        use $app as App;
        use $crate::text::{Action, Host};
        use $input as InputMode;
        use $mode as ViewMode;
        use $modifiers as KeyModifiers;
        $crate::__frontend_text_buffer!();
        $crate::__frontend_text_filters!($push, $backspace);
        $crate::__frontend_text_symbols!($dismiss, $open, $refresh);
        pub(super) struct Native<'a>(pub &'a mut App);
        impl Host for Native<'_> {
            fn symbols_active(&self) -> bool {
                $active(self.0)
            }
            fn in_view(&self, view: ViewMode) -> bool {
                self.0.state.view_mode == view
            }
            fn bus_filter_active(&self) -> bool {
                self.0.state.bus_log.filter_input_mode
            }
            fn bus_filter(&self) -> &str {
                &self.0.state.bus_log.filter
            }
            fn input(&self) -> &str {
                &self.0.state.input
            }
            fn set_status(&mut self, status: String) {
                self.0.state.status = status;
            }
            fn set_mode(&mut self, mode: InputMode) {
                self.0.state.input_mode = mode;
            }
            fn effect(&mut self, action: Action) {
                if buffer::apply(self.0, &action)
                    || filters::apply(self.0, &action)
                    || symbols::apply(self.0, &action)
                {
                    return;
                }
                unreachable!("unmapped text input effect");
            }
        }
        /// Apply one character event using the shared routing policy.
        pub async fn handle_char(app: &mut App, modifiers: KeyModifiers, c: char) {
            $crate::text::character(&mut Native(app), modifiers, c);
        }
        /// Apply backspace using the shared routing policy.
        pub async fn handle_backspace(app: &mut App) {
            $crate::text::backspace(&mut Native(app));
        }
    };
}
