//! Frontend timing operations for tool events.
/// Supporting implementation for [crate::declare_tool_events].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_tool_event_timing {
    ($native:ident, $started:path) => {
        impl $crate::tools::Timing for $native<'_> {
            async fn processing_started(&mut self) {
                $started(self.app, self.worker_bridge).await;
            }
            fn request_timing_started(&self) -> bool {
                self.app
                    .state
                    .request_timing
                    .processing_started_at
                    .is_some()
            }
            fn begin_request_timing(&mut self) {
                self.app.state.begin_request_timing();
            }
            fn streaming_started(&self) -> bool {
                self.app.state.streaming.streaming_start.is_some()
            }
            fn begin_streaming(&mut self) {
                self.app.state.streaming.begin_streaming();
            }
        }
    };
}
