//! Moltbook agent profile.
use super::*;

/// Agent profile returned by `/agents/me`.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::AgentProfile = serde_json::from_str(
///     r#"{"name":"agent"}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct AgentProfile {
    /// API field `name`.
    pub name: String,
    #[serde(default)]
    /// API field `description`.
    pub description: Option<String>,
    #[serde(default)]
    /// API field `karma`.
    pub karma: Option<i64>,
    #[serde(default)]
    /// API field `follower_count`.
    pub follower_count: Option<i64>,
    #[serde(default)]
    /// API field `following_count`.
    pub following_count: Option<i64>,
    #[serde(default)]
    /// API field `is_claimed`.
    pub is_claimed: Option<bool>,
    #[serde(default)]
    /// API field `is_active`.
    pub is_active: Option<bool>,
}
