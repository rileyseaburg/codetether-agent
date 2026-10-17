//! Compatibility exports for shared registry-provider requests.
pub(super) use codetether_cognition_core::thinker::protocol::provider_request::{
    build, validate_model,
};

#[cfg(test)]
mod tests {
    use super::validate_model;

    #[test]
    fn accepts_gpt_56_from_multiple_providers() {
        assert!(validate_model("gpt-5.6-sol-fast:max").is_ok());
        assert!(validate_model("openai.gpt-5.6-sol").is_ok());
        assert!(validate_model("claude-sonnet-4").is_err());
    }
}
