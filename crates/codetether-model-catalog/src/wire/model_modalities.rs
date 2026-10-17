//! API catalog ModelModalities representation.
use super::*;

/// Model modalities
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let modes = codetether_model_catalog::ModelModalities { input: vec!["text".into()], output: vec!["text".into()] };
/// assert_eq!(modes.input[0], "text");
/// ```
pub struct ModelModalities {
    #[serde(default)]
    /// Advertised input modalities.
    pub input: Vec<String>,
    #[serde(default)]
    /// Advertised output modalities.
    pub output: Vec<String>,
}
