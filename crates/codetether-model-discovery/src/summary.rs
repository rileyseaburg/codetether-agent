//! Provider discovery counts and outcomes.

/// Outcome counts from one provider discovery pass.
/// # Examples
/// ```
/// let summary = codetether_model_discovery::ModelRefreshSummary::empty();
/// assert_eq!(summary.loaded_models, 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRefreshSummary {
    /// Number of provider names returned by the registry.
    pub provider_count: usize,
    /// Providers contributing at least one selectable model.
    pub loaded_providers: usize,
    /// Unique selectable model references.
    pub loaded_models: usize,
    /// Providers whose model request returned an error.
    pub failed_providers: usize,
    /// Providers whose request exceeded the discovery timeout.
    pub timed_out_providers: usize,
}
impl ModelRefreshSummary {
    /// Creates a summary with every counter set to zero.
    pub fn empty() -> Self {
        Self {
            provider_count: 0,
            loaded_providers: 0,
            loaded_models: 0,
            failed_providers: 0,
            timed_out_providers: 0,
        }
    }
}
