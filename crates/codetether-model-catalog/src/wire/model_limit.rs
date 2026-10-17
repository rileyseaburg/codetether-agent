//! API catalog ModelLimit representation.
use super::*;

/// Model limits
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let limit = codetether_model_catalog::ModelLimit { context: 128000, output: 8192 };
/// assert!(limit.context > limit.output);
/// ```
pub struct ModelLimit {
    #[serde(default)]
    /// Advertised context-token limit, or zero when absent.
    pub context: u64,
    #[serde(default)]
    /// Advertised output-token limit, or zero when absent.
    pub output: u64,
}
