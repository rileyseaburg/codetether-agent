//! Frontend watchdog feedback and timing state.
/// Implementation detail of [crate::declare_frontend_watchdog].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_watchdog_ui {
    ($app:path) => {
        mod ui {
            use $app as App;
            use $crate::recovery::{State, Ui};
            impl Ui for App {
                fn state(&mut self) -> State<'_> {
                    State {
                        notification: &mut self.state.watchdog.notification,
                        processing: &mut self.state.processing,
                        restarts: &mut self.state.watchdog.restart_count,
                        inflight: &mut self.state.watchdog.inflight_prompt,
                        last_event: &mut self.state.watchdog.last_event_at,
                        status: &mut self.state.status,
                        messages: &mut self.state.messages,
                    }
                }
                fn clear_streaming(&mut self) {
                    self.state.clear_streaming_text();
                }
                fn clear_timing(&mut self) {
                    self.state.clear_request_timing();
                }
                fn begin_timing(&mut self) {
                    self.state.begin_request_timing();
                }
                fn scroll(&mut self) {
                    self.state.scroll_to_bottom();
                }
            }
        }
    };
}
