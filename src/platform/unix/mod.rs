//! Unix platform stubs.
//!
//! On non-Windows platforms, native OS APIs are not available.
//! Audio capture/playback falls back to `cpal` + `hound`.

/// Unix has no WASAPI — always returns `None`.
pub fn native_voice_capture() -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
    None
}
