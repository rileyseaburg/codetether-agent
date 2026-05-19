/// Provider availability and model context information.
#[derive(Debug, Clone)]
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
