//! Resolution of the RLM subcall model selector against the provider registry.

use crate::provider::ProviderRegistry;
use crate::session::Session;

impl Session {
    /// Reset and, when a registry is available, resolve the subcall model
    /// selector carried by `metadata.rlm`.
    pub(super) fn apply_subcall_model(&mut self, registry: Option<&ProviderRegistry>) {
        self.metadata.subcall_provider = None;
        self.metadata.subcall_model_name = None;
        if let Some(registry) = registry {
            self.resolve_subcall_provider(registry);
        }
    }

    /// Attempt to resolve [`RlmConfig::subcall_model`] against the given
    /// provider registry, storing the result on metadata.
    ///
    /// Called by session helpers right before building an
    /// [`AutoProcessContext`](crate::rlm::router::AutoProcessContext) if
    /// `subcall_provider` is still `None` but `subcall_model` is configured.
    /// This deferred resolution avoids requiring the registry at session
    /// creation time.
    ///
    /// # Errors
    ///
    /// Does **not** return errors — resolution failure is logged.
    ///
    /// [`RlmConfig::subcall_model`]: crate::config::RlmConfig::subcall_model
    pub fn resolve_subcall_provider(&mut self, registry: &ProviderRegistry) {
        if self.metadata.subcall_provider.is_some() {
            return;
        }
        let Some(selector) = self.metadata.rlm.subcall_model.as_deref() else {
            return;
        };
        match registry.resolve_model(selector) {
            Ok((provider, model_name)) => {
                tracing::debug!(subcall_model = %model_name, "RLM: resolved subcall provider");
                self.metadata.subcall_provider = Some(provider);
                self.metadata.subcall_model_name = Some(model_name);
            }
            Err(error) => tracing::warn!(
                configured = selector,
                %error,
                "RLM subcall_model resolution failed; subcalls will use root model"
            ),
        }
    }
}
