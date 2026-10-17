//! MiniMax-specific error classification helpers.

/// True when MiniMax rejected chat settings (error code 2013).
pub(super) fn is_minimax_chat_setting_error(error: &str) -> bool {
    let normalized = error.to_ascii_lowercase();
    normalized.contains("invalid chat setting")
        || normalized.contains("(2013)")
        || normalized.contains("code: 2013")
        || normalized.contains("\"2013\"")
}
