//! Retry markers for interrupted Z.AI streaming transports.

/// Prefix used when SRP should restart the Z.AI stream.
pub const RETRYABLE_PREFIX: &str = "zai-retryable: ";

/// Prefix a transport error so session SRP treats it as restartable.
pub fn retryable(error: reqwest::Error) -> String {
    format!("{RETRYABLE_PREFIX}Z.AI stream interrupted: {error}")
}

#[cfg(test)]
mod tests {
    use super::RETRYABLE_PREFIX;

    #[test]
    fn retryable_prefix_matches_session_classifier() {
        assert_eq!(RETRYABLE_PREFIX, "zai-retryable: ");
    }
}
