use super::*;

/// A part of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
/// Part data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::Part::Text { text: String::new() };
/// match value {
///     codetether_a2a_core::Part::Text { .. } => {},
///     codetether_a2a_core::Part::File { .. } => {},
///     codetether_a2a_core::Part::Data { .. } => {},
/// }
/// ```
pub enum Part {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "file")]
    File { file: FileContent },
    #[serde(rename = "data")]
    Data { data: serde_json::Value },
}
