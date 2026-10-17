//! Moltbook post.
use super::*;

/// A single Moltbook post.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::Post = serde_json::from_str(
///     r#"{"id":"post-1"}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct Post {
    /// API field `id`.
    pub id: String,
    #[serde(default)]
    /// API field `title`.
    pub title: Option<String>,
    #[serde(default)]
    /// API field `content`.
    pub content: Option<String>,
    #[serde(default)]
    /// API field `upvotes`.
    pub upvotes: Option<i64>,
    #[serde(default)]
    /// API field `downvotes`.
    pub downvotes: Option<i64>,
    #[serde(default)]
    /// API field `author`.
    pub author: Option<PostAuthor>,
}
