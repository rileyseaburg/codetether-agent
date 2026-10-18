//! Frontend error state access.
/// Supporting implementation for [crate::declare_error_events].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_error_view {
    ($native:ident, $settle:path) => {
        mod view {
            use super::$native;
            impl $crate::error::View for $native<'_> {
                fn settle_error(&mut self, error: String) {
                    $settle(self.app, Err(error));
                }
                fn complete_request_timing(&mut self) {
                    self.app.state.complete_request_timing();
                }
                fn schedule_retry(&mut self, error: &str) {
                    super::smart_switch::schedule(self.app, self.slot, error);
                }
                fn messages(&mut self) -> &mut Vec<$crate::__private::ChatMessage> {
                    &mut self.app.state.messages
                }
                fn auto_follow(&self) -> bool {
                    self.app.state.chat_viewport.chat_auto_follow
                }
                fn scroll_to_bottom(&mut self) {
                    self.app.state.scroll_to_bottom();
                }
            }
        }
    };
}
