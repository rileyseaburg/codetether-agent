//! Select the legacy buffered adapter only when genuine streaming is unverified.
use crate::provider::BedrockProvider;

impl BedrockProvider {
    /// Retain InvokeModel streaming emulation for older/unknown Fable versions.
    /// Only the measured US Fable 5.2 profile opts into Converse streaming.
    /// Other profiles, aliases and future suffixes retain their existing route
    /// until their support is established independently.
    pub(in crate::provider) fn should_use_invoke_model_stream(model_id: &str) -> bool {
        Self::should_use_invoke_model(model_id) && model_id != "us.anthropic.claude-fable-5-2"
    }
}

#[cfg(test)]
mod tests;
