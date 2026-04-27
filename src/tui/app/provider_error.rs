//! User-facing provider error messages.

const CONTEXT_OVERFLOW: &str = "The conversation was too large for the selected model, so CodeTether compacted context and retried. If this repeats, start a fresh session or use /context_reset.";

/// Returns true when a provider error is a context-window overflow.
pub fn is_context_overflow_error(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();
    normalized.contains("context window")
        || normalized.contains("context length")
        || normalized.contains("maximum context")
        || normalized.contains("prompt is too long")
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
