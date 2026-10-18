//! Frontend completion operations for session-event lifecycles.
/// Supporting implementation for [crate::declare_event_lifecycle].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_event_completion {
    ($native:ident, $settle:path) => {
        impl $crate::lifecycle::Completion for $native<'_> {
            fn settle_inbox(&mut self) {
                $settle(self.app, self.slot);
            }
            fn clear_streaming_text(&mut self) {
                self.app.state.clear_streaming_text();
            }
            fn complete_turn_timing(&mut self) {
                self.app.state.complete_turn_timing();
            }
            fn clear_streaming_start(&mut self) {
                self.app.state.streaming.streaming_start = None;
            }
            fn clear_interlude(&mut self) {
                self.app.state.interlude = None;
            }
            fn clear_watchdog_notification(&mut self) {
                self.app.state.watchdog.notification = None;
            }
            fn clear_inflight_prompt(&mut self) {
                self.app.state.watchdog.inflight_prompt = None;
            }
        }
    };
}
