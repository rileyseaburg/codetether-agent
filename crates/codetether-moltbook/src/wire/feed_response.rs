//! Moltbook feed response.
use super::*;

/// Feed response.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::FeedResponse = serde_json::from_str(
///     r#"{"success":true,"posts":[]}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct FeedResponse {
    #[serde(default)]
    /// API field `success`.
    pub success: bool,
    #[serde(default)]
    /// API field `posts`.
    pub posts: Vec<Post>,
}
