/// Provider availability and model context information.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let status = codetether_swarm_core::validation::ProviderStatus { provider: "test".into(), model: "model".into(), is_available: true, has_credentials: true, context_window: 128_000 };
/// assert!(status.is_available);
/// ```
pub struct ProviderStatus {
    /// Selected provider name.
    pub provider: String,
    /// Selected model name.
    pub model: String,
    /// Whether the selected provider is available.
    pub is_available: bool,
    /// Whether credentials are configured.
    pub has_credentials: bool,
    /// Selected model context window size.
    pub context_window: usize,
}
