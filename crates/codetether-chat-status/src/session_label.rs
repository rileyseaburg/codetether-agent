//! Temporary model label while a request is processing.

/// Return the temporary model label while processing.
///
/// # Arguments
/// * `processing` - Whether a request is active.
///
/// # Returns
/// The processing label when active, or none while idle.
///
/// # Examples
/// ```
/// assert_eq!(codetether_chat_status::session_model_label(false), None);
/// ```
pub fn session_model_label(processing: bool) -> Option<String> {
    if processing {
        Some("processing".to_string())
    } else {
        None
    }
}
