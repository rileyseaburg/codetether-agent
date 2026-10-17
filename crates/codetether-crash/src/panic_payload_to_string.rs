//! Crash reporting panic payload to string.
use super::*;
pub(super) fn panic_payload_to_string(panic_info: &panic::PanicHookInfo<'_>) -> String {
    if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
        (*msg).to_string()
    } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
        msg.clone()
    } else {
        "non-string panic payload".to_string()
    }
}
