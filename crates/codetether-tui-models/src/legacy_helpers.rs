//! Legacy UI helper surface retained for compatibility.
//!
//! These marker types and fallback values preserve the original TUI utility
//! API. Environment classification uses the original explicit-flag precedence.
pub use super::legacy_environment::is_secure_environment;
/// Return the input unchanged, preserving the legacy helper behavior.
/// # Arguments
/// * `s` - Text to copy.
/// * `_max` - Retained compatibility argument; currently ignored.
/// # Returns
/// An owned copy of the complete input.
/// # Examples
/// ```
/// assert_eq!(codetether_tui_models::legacy_helpers::truncate_with_ellipsis("hello", 2), "hello");
/// ```
pub fn truncate_with_ellipsis(s: &str, _max: usize) -> String {
    s.to_string()
}

/// Legacy `PendingImage` marker with no stored state.
/// # Examples
/// ```
/// let _marker = codetether_tui_models::legacy_helpers::PendingImage;
/// ```
pub struct PendingImage;
/// Legacy `ChatSyncConfig` marker with no stored state.
/// # Examples
/// ```
/// let _marker = codetether_tui_models::legacy_helpers::ChatSyncConfig;
/// ```
pub struct ChatSyncConfig;
/// Legacy `MinioClient` marker with no stored state.
/// # Examples
/// ```
/// let _marker = codetether_tui_models::legacy_helpers::MinioClient;
/// ```
pub struct MinioClient;

/// Return the legacy chat-sync configuration fallback.
/// # Arguments
/// None.
/// # Returns
/// Always `None`; configuration is not loaded by this helper.
/// # Examples
/// ```
/// assert!(codetether_tui_models::legacy_helpers::parse_chat_sync_config().is_none());
/// ```
pub fn parse_chat_sync_config() -> Option<ChatSyncConfig> {
    None
}
/// Return the legacy byte-count display fallback.
/// # Arguments
/// * `_n` - Retained compatibility argument; currently ignored.
/// # Returns
/// The string `"0B"` for every input.
/// # Examples
/// ```
/// assert_eq!(codetether_tui_models::legacy_helpers::format_bytes(100), "0B");
/// ```
pub fn format_bytes(_n: u64) -> String {
    "0B".to_string()
}
/// Return the legacy static spinner frame.
/// # Arguments
/// None.
/// # Returns
/// The static string `"|"`.
/// # Examples
/// ```
/// assert_eq!(codetether_tui_models::legacy_helpers::current_spinner_frame(), "|");
/// ```
pub fn current_spinner_frame() -> &'static str {
    "|"
}

/// Legacy `ChatArchiveRecord` marker with no stored state.
/// # Examples
/// ```
/// let _marker = codetether_tui_models::legacy_helpers::ChatArchiveRecord;
/// ```
pub struct ChatArchiveRecord;
