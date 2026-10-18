//! Default-model seeding regressions using session metadata.

#[cfg(test)]
mod tests {
    use crate::seed_default_model;
    use codetether_config::Config;
    use codetether_session_metadata::SessionMetadata;

    #[test]
    fn fresh_session_adopts_the_configured_default_model() {
        let mut config = Config::default();
        config.default_model = Some("openai-codex/gpt-6-astra-fast:high".into());
        let mut metadata = SessionMetadata::default();
        seed_default_model(&mut metadata, &config);
        assert_eq!(
            metadata.model.as_deref(),
            Some("openai-codex/gpt-6-astra-fast:high")
        );
    }

    #[test]
    fn explicit_session_model_is_never_overridden() {
        let mut config = Config::default();
        config.default_model = Some("openai-codex/gpt-6-astra-fast:high".into());
        let mut metadata = SessionMetadata::default();
        metadata.model = Some("bedrock/claude".into());
        seed_default_model(&mut metadata, &config);
        assert_eq!(metadata.model.as_deref(), Some("bedrock/claude"));
    }

    #[test]
    fn blank_default_leaves_the_session_unset() {
        let mut config = Config::default();
        config.default_model = Some("   ".into());
        let mut metadata = SessionMetadata::default();
        seed_default_model(&mut metadata, &config);
        assert!(metadata.model.is_none());
    }
}
