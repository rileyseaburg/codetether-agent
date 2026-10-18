//! Application configuration binding for the shared session lifecycle.
use super::types::Session;
#[path = "lifecycle/subcall_model.rs"]
mod subcall_model;

impl Session {
    /// Seed session metadata from a loaded [`crate::config::Config`].
    ///
    /// Copies [`crate::config::Config::rlm`] into [`SessionMetadata::rlm`](super::types::SessionMetadata::rlm) so
    /// RLM compaction and tool-output routing honour user-configured
    /// thresholds, iteration limits, and model selectors, and adopts
    /// [`crate::config::Config::default_model`] when the session has no model
    /// yet so the TUI honours `CODETETHER_DEFAULT_MODEL` / `default_model`
    /// exactly like `codetether run`.
    ///
    /// Also attempts to resolve [`RlmConfig::subcall_model`] against the
    /// given provider registry. When resolution succeeds the resolved
    /// provider is cached on [`SessionMetadata`](super::types::SessionMetadata) (not serialised) so
    /// every `AutoProcessContext` built from this session can cheaply
    /// reference it. On failure the subcall provider is left as `None`
    /// and the resolution failure is logged.
    ///
    /// Idempotent: re-applying the same config is a no-op.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::config::Config;
    /// use codetether_agent::session::Session;
    ///
    /// let cfg = Config::default();
    /// let mut session = Session::new().await.unwrap();
    /// session.apply_config(&cfg, None);
    /// assert_eq!(session.metadata.rlm.mode, cfg.rlm.mode);
    /// # });
    /// ```
    pub fn apply_config(
        &mut self,
        config: &crate::config::Config,
        registry: Option<&crate::provider::ProviderRegistry>,
    ) {
        codetether_session_model_policy::apply_config(&mut self.metadata, config, registry);
    }
}

codetether_session_schema::impl_session_lifecycle!(Session);
