//! User-facing provider error messages.

const CONTEXT_OVERFLOW: &str = "The conversation was too large for the selected model, so CodeTether compacted context and retried. If this repeats, start a fresh session with /new.";

/// Returns true when a provider error is a context-window overflow.
pub fn is_context_overflow_error(message: &str) -> bool {
    crate::session::helper::error_detect::is_prompt_too_long_message(message)
}

/// Convert raw upstream errors into messages safe to show users.
pub fn user_facing_error(message: &str) -> String {
    if is_context_overflow_error(message) {
        CONTEXT_OVERFLOW.to_string()
    } else {
        message.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hides_raw_context_window_error() {
        let raw = "Your input exceeds the context window of this model.";
        let shown = user_facing_error(raw);
        assert!(!shown.contains("Your input exceeds"));
        assert!(shown.contains("conversation was too large"));
    }
}
