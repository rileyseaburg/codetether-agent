//! API catalog ProviderInfo representation.
use super::*;

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let provider = codetether_model_catalog::ProviderInfo {
///     id: "example".into(), name: "Example".into(), env: vec![], npm: None,
///     api: None, doc: None, models: std::collections::HashMap::new(),
/// };
/// assert!(provider.models.is_empty());
/// ```
pub struct ProviderInfo {
    /// Provider identifier supplied by the catalog.
    pub id: String,
    #[serde(default)]
    /// Associated credential environment variable names.
    pub env: Vec<String>,
    #[serde(default)]
    /// Optional client package identifier.
    pub npm: Option<String>,
    #[serde(default)]
    /// Optional API endpoint metadata.
    pub api: Option<String>,
    /// Provider display name.
    pub name: String,
    #[serde(default)]
    /// Optional documentation URL.
    pub doc: Option<String>,
    #[serde(default)]
    /// Models indexed by catalog identifier.
    pub models: HashMap<String, ApiModelInfo>,
}
