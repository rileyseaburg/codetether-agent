//! Application-local tui-navigation state adapters.
pub use codetether_tui_models::ViewMode;
/// Declare the existing tui-navigation methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! impl_app_view_mode {
    ($state:ty) => {
        impl $state {
            pub fn set_view_mode(&mut self, view_mode: $crate::state_binding::ViewMode) {
                $crate::view_state::set_view_mode(
                    &mut self.view_mode,
                    view_mode,
                    &mut self.saved_scroll,
                    self.chat_viewport.chat_scroll,
                    self.chat_viewport.chat_auto_follow,
                    self.tool_viewport.tool_preview_scroll,
                );
            }
        }
    };
}
