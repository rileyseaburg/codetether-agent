//! Response envelope and choice wire types.
use crate::*;
mod message;
mod usage;
pub(crate) use message::*;
pub(crate) use usage::*;
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterResponse {
    #[serde(default)]
    pub(crate) id: String,
    // provider and model fields from OpenRouter
    #[serde(default)]
    pub(crate) provider: Option<String>,
    #[serde(default)]
    pub(crate) model: Option<String>,
    pub(crate) choices: Vec<OpenRouterChoice>,
    #[serde(default)]
    pub(crate) usage: Option<OpenRouterUsage>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterChoice {
    pub(crate) message: OpenRouterMessage,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
    // OpenRouter adds native_finish_reason
    #[serde(default)]
    pub(crate) native_finish_reason: Option<String>,
}
