//! Vertex response schema.
use crate::*;
mod content;
mod error;
mod usage;
pub(crate) use content::*;
pub(crate) use error::*;
pub(crate) use usage::*;
// Response types (same as Anthropic)
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicResponse {
    #[allow(dead_code)]
    pub(crate) id: String,
    #[allow(dead_code)]
    pub(crate) model: String,
    pub(crate) content: Vec<AnthropicContent>,
    #[serde(default)]
    pub(crate) stop_reason: Option<String>,
    #[serde(default)]
    pub(crate) usage: Option<AnthropicUsage>,
}
