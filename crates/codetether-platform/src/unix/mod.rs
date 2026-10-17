//! Unix platform stubs.
//!
//! On non-Windows platforms, native OS APIs are not available.
//! Audio capture/playback falls back to `cpal` + `hound`.

/// Unix has no WASAPI — always returns `None`.
/// # Returns
/// `None`, because this module does not supply a native audio capture implementation.
///
/// # Examples
/// ```
/// assert!(codetether_platform::unix::native_voice_capture().is_none());
/// ```
pub fn native_voice_capture() -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
    None
}
