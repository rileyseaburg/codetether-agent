//! Native configuration-policy integration tests.

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::session::Session;

    #[tokio::test]
    async fn fresh_session_adopts_the_configured_default_model() {
        let mut config = Config::default();
        config.default_model = Some("openai-codex/gpt-6-astra-fast:high".into());
        let mut session = Session::new().await.unwrap();
        session.apply_config(&config, None);
        assert_eq!(
            session.metadata.model.as_deref(),
            Some("openai-codex/gpt-6-astra-fast:high")
        );
    }

    #[tokio::test]
    async fn explicit_session_model_is_never_overridden() {
        let mut config = Config::default();
        config.default_model = Some("openai-codex/gpt-6-astra-fast:high".into());
        let mut session = Session::new().await.unwrap();
        session.metadata.model = Some("bedrock/claude".into());
        session.apply_config(&config, None);
        assert_eq!(session.metadata.model.as_deref(), Some("bedrock/claude"));
    }

    #[tokio::test]
    async fn blank_default_leaves_the_session_unset() {
        let mut config = Config::default();
        config.default_model = Some("   ".into());
        let mut session = Session::new().await.unwrap();
        session.apply_config(&config, None);
        assert!(session.metadata.model.is_none());
    }
}
