//! Moltbook api response.
use super::*;

/// Generic success wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::ApiResponse<serde_json::Value> = serde_json::from_str(
///     r#"{"success":true}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct ApiResponse<T> {
    #[serde(default)]
    /// API field `success`.
    pub success: bool,
    #[serde(default)]
    /// API field `error`.
    pub error: Option<String>,
    #[serde(default)]
    /// API field `hint`.
    pub hint: Option<String>,
    #[serde(flatten)]
    /// API field `data`.
    pub data: T,
}
