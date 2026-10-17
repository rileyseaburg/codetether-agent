//! manifest for verified plugin registration.
use super::*;

/// Manifest describing a plugin tool.
///
/// # Examples
/// ```rust
/// let manifest = codetether_plugin_verification::PluginManifest {
///     id: "example".into(), name: "Example".into(), version: "1.0.0".into(),
///     content_hash: String::new(), signed_by: "owner".into(),
///     signature: String::new(), capabilities: vec![], timeout_secs: 30,
/// };
/// assert_eq!(manifest.timeout_secs, 30);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// SHA-256 hash of the plugin content (source or binary).
    pub content_hash: String,
    /// Who signed this manifest.
    pub signed_by: String,
    /// Hex-encoded HMAC-SHA256 signature of `id|version|content_hash`.
    pub signature: String,
    /// Allowed capabilities (e.g., "fs:read", "net:connect", "exec:shell").
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Maximum execution time in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}
