//! Moltbook post author.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::PostAuthor = serde_json::from_str(
///     r#"{"name":"agent"}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct PostAuthor {
    /// API field `name`.
    pub name: String,
}
