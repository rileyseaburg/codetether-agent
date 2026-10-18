//! Typed token gate for Bedrock auth output.

/// Bedrock bearer token that is allowed to be printed or saved.
pub(super) struct OutputToken {
    value: String,
}

impl OutputToken {
    /// Create an output token after the validation policy has completed.
    pub(super) fn new(value: String) -> Self {
        Self { value }
    }

    /// Return the bearer token string.
    pub(super) fn expose(&self) -> &str {
        &self.value
    }
}
