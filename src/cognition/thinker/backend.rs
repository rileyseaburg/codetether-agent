//! Backend selection for cognition inference.

/// Available cognition inference backends.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::thinker::ThinkerBackend;
/// assert_eq!(ThinkerBackend::from_env("registry"), ThinkerBackend::Registry);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinkerBackend {
    /// Generic OpenAI-compatible HTTP endpoint.
    OpenAICompat,
    /// Authenticated model provider selected from the shared registry.
    Registry,
    /// In-process Candle model runtime.
    Candle,
    /// Amazon Bedrock Converse provider.
    Bedrock,
}

impl ThinkerBackend {
    /// Parse an environment configuration value.
    ///
    /// # Arguments
    ///
    /// * `value` — Backend name from `CODETETHER_COGNITION_THINKER_BACKEND`.
    ///
    /// # Returns
    ///
    /// The matching backend, defaulting to OpenAI-compatible HTTP.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::cognition::thinker::ThinkerBackend;
    /// assert_eq!(ThinkerBackend::from_env("provider"), ThinkerBackend::Registry);
    /// ```
    pub fn from_env(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "candle" => Self::Candle,
            "codex" | "openai_codex" | "openai-codex" | "provider" | "registry" => Self::Registry,
            "bedrock" | "aws" | "aws_bedrock" => Self::Bedrock,
            _ => Self::OpenAICompat,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThinkerBackend;

    #[test]
    fn keeps_bedrock_and_registry_backends() {
        assert_eq!(ThinkerBackend::from_env("bedrock"), ThinkerBackend::Bedrock);
        assert_eq!(
            ThinkerBackend::from_env("registry"),
            ThinkerBackend::Registry
        );
    }
}
