//! AppState pending_tool adapter using existing fields and operations.
/// Declare the existing pending_tool methods on the local view state.
/// # Usage
/// ```text
/// codetether_telemetry::__app_tool_timing!(AppState);
/// ```
#[macro_export]
macro_rules! __app_tool_timing {
    ($state:ty) => {
        impl $state {
            /// Mark `name` as the currently-running tool and record when it started.
            pub fn start_pending_tool(&mut self, name: ::std::string::String) {
                self.tool_timing.start_pending_tool(name);
            }

            /// Record the tool result fields and clear the in-flight marker.
            pub fn note_tool_completed(
                &mut self,
                name: ::std::string::String,
                duration_ms: u64,
                success: bool,
            ) {
                self.tool_timing
                    .note_tool_completed(name, duration_ms, success);
            }

            /// Borrowed view of the in-flight tool, if any: `(name, started_at)`.
            pub fn pending_tool_snapshot(&self) -> Option<(&str, ::std::time::Instant)> {
                self.tool_timing.pending_tool_snapshot()
            }
        }
    };
}
