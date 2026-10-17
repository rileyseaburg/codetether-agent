//! API catalog ModelCost representation.
use super::*;

/// Model cost information (per million tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let cost = codetether_model_catalog::ModelCost { input: 1.0, output: 2.0, cache_read: None, cache_write: None, reasoning: None };
/// assert_eq!(cost.input, 1.0);
/// ```
pub struct ModelCost {
    /// Input-token cost per million.
    pub input: f64,
    /// Output-token cost per million.
    pub output: f64,
    #[serde(default)]
    /// Optional cache-read token cost.
    pub cache_read: Option<f64>,
    #[serde(default)]
    /// Optional cache-write token cost.
    pub cache_write: Option<f64>,
    #[serde(default)]
    /// Optional reasoning-token cost.
    pub reasoning: Option<f64>,
}
