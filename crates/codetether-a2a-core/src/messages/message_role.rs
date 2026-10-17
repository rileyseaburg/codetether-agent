use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// MessageRole data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::MessageRole::User;
/// match value {
///     codetether_a2a_core::MessageRole::User => {},
///     codetether_a2a_core::MessageRole::Agent => {},
/// }
/// ```
pub enum MessageRole {
    User,
    Agent,
}
