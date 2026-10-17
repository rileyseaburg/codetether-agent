use super::*;

/// Task query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// TaskQueryParams data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskQueryParams { id: String::new(), history_length: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct TaskQueryParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<usize>,
}
