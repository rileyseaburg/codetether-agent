//! Existing request, watchdog and pending-tool timing on local UI state.
pub mod request;
pub mod tool;
/// Declare request and pending-tool timing methods on local application state.
/// # Usage
/// ```text
/// codetether_telemetry::impl_app_timing!(AppState);
/// ```
#[macro_export]
macro_rules! impl_app_timing {
    ($state:ty) => {
        $crate::__app_request_timing!($state);
        $crate::__app_tool_timing!($state);
    };
}
