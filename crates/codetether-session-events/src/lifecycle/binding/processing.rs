//! Frontend processing operations for session-event lifecycles.
/// Supporting implementation for [crate::declare_event_lifecycle].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_event_processing {
    ($native:ident, $started:path, $stopped:path) => {
        impl $crate::lifecycle::Processing for $native<'_> {
            async fn processing_started(&mut self) {
                $started(self.app, self.worker_bridge).await;
            }
            async fn processing_stopped(&mut self) {
                $stopped(self.app, self.worker_bridge).await;
            }
            fn request_timing_started(&self) -> bool {
                self.app
                    .state
                    .request_timing
                    .processing_started_at
                    .is_some()
            }
            fn begin_request_timing(&mut self) {
                self.app
                    .state
                    .request_timing
                    .begin_with_activity(&mut self.app.state.watchdog.last_event_at);
            }
            fn streaming_started(&self) -> bool {
                self.app.state.streaming.streaming_start.is_some()
            }
            fn begin_streaming(&mut self) {
                self.app.state.streaming.begin_streaming();
            }
            fn set_status(&mut self, status: String) {
                self.app.state.status = status;
            }
        }
    };
}
