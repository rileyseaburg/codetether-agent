use super::*;

/// File content (bytes or URI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// FileContent data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::FileContent { bytes: None, uri: None, mime_type: None, name: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct FileContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>, // Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
