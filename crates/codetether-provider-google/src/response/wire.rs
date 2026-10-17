//! Completion envelope and choice schema.
use crate::*;
mod message;
mod signatures;
mod usage;
pub(crate) use message::*;
pub(crate) use signatures::*;
pub(crate) use usage::*;
/// OpenAI-compatible types for parsing Google's response

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletion {
    #[allow(dead_code)]
    pub(crate) id: Option<String>,
    pub(crate) choices: Vec<Choice>,
    #[serde(default)]
    pub(crate) usage: Option<ApiUsage>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    pub(crate) message: ChoiceMessage,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
