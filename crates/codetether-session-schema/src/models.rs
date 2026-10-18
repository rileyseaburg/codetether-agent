//! Apply model configuration through the application's existing registry type.
/// Add configuration and subcall-provider methods to a local session.
///
/// The registry must implement the shared model policy's ModelResolver contract.
/// # Usage
/// ```text
/// codetether_session_schema::impl_session_models!(Session, ProviderRegistry);
/// ```
#[macro_export]
macro_rules! impl_session_models {
    ($name:ident, $registry:ty) => {
        impl $name {
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
                config: &$crate::__private::config::Config,
                registry: ::std::option::Option<&$registry>,
            ) {
                $crate::__private::model_policy::apply_config(&mut self.metadata, config, registry);
            }
        }
        impl $name {
            /// Resolve the configured subcall model unless a provider is already cached.
            pub fn resolve_subcall_provider(&mut self, registry: &$registry) {
                $crate::__private::model_policy::resolve_subcall_provider(
                    &mut self.metadata,
                    registry,
                );
            }
        }
    };
}
