//! PromptContent wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PromptContent {
    Text { text: String },
    Image { data: String, mime_type: String },
    Resource { resource: ResourceContents },
}
