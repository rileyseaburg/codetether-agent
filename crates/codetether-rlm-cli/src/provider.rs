//! Host-owned provider initialization for the RLM command.
/// Supplies one initialized provider after input preparation.
/// # Implementors
/// The application preserves its Vault lookup and provider-selection policy.
#[async_trait::async_trait]
pub trait ProviderSource {
    /// Resolve a provider using the requested CLI model.
    /// # Errors
    /// Returns credential-loading or model/provider availability failures.
    async fn select(
        model: Option<&str>,
    ) -> anyhow::Result<codetether_rlm_selection::SelectedProvider>;
}
