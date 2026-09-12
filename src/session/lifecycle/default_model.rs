//! Seeding a session's model selector from the loaded configuration.
//!
//! `codetether run` resolves `CLI arg > CODETETHER_DEFAULT_MODEL > config`
//! before its first prompt. The TUI never did: a fresh session reached the
//! prompt loop with no model and fell back to the per-provider hardcoded
//! default (`gpt-5.5` for `openai-codex`), silently ignoring the user's
//! configured default. This module closes that gap for every caller of
//! [`Session::apply_config`](crate::session::Session::apply_config).

use crate::session::Session;

impl Session {
    /// Adopt `config.default_model` when the session has no model yet.
    ///
    /// Explicit choices always win: a model picked in the TUI, passed on the
    /// command line, or restored from a persisted session is left untouched.
    pub(super) fn seed_default_model(&mut self, config: &crate::config::Config) {
        if self.metadata.model.is_some() {
            return;
        }
        let Some(model) = config.default_model.as_deref().map(str::trim) else {
            return;
        };
        if model.is_empty() {
            return;
        }
        tracing::info!(model, "Seeding session model from configured default");
        self.metadata.model = Some(model.to_string());
    }
}

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
