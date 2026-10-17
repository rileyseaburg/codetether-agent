use super::*;

/// An artifact produced by a task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Artifact data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::Artifact { artifact_id: String::new(), parts: vec![], name: None, description: None, metadata: std::collections::HashMap::new(), extensions: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct Artifact {
    pub artifact_id: String,
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// URIs of extensions present in or contributing to this Artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,
}
