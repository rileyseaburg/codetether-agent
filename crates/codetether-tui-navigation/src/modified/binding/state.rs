//! Frontend state effects for modifier shortcuts.
/// Implementation detail of [crate::declare_frontend_modified_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_modified_state {
    ($mode:path, $layout:path) => {
        mod state {
            use super::Native;
            use $crate::modified::State;
            use $layout as ChatLayoutMode;
            use $mode as ViewMode;
            impl State for Native<'_> {
                fn in_chat(&self) -> bool {
                    self.app.state.view_mode == ViewMode::Chat
                }
                fn watchdog_active(&self) -> bool {
                    self.app.state.watchdog.notification.is_some()
                }
                fn webview(&self) -> bool {
                    self.app.state.chat_layout_mode == ChatLayoutMode::Webview
                }
                fn status(&mut self) -> &mut String {
                    &mut self.app.state.status
                }
            }
        }
    };
}
