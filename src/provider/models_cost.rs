//! Per-token model pricing metadata.

use serde::{Deserialize, Serialize};

/// Model costs per million tokens.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::models::ModelCost;
/// let cost = ModelCost {
///     input: 1.0, output: 2.0, cache_read: None, cache_write: None, reasoning: None,
/// };
/// assert_eq!(cost.output, 2.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCost {
    pub input: f64,
    pub output: f64,
    #[serde(default)]
    pub cache_read: Option<f64>,
    #[serde(default)]
    pub cache_write: Option<f64>,
    #[serde(default)]
    pub reasoning: Option<f64>,
}
