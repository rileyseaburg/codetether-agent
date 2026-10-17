//! Buffered response wire data.
use crate::*;
mod message;
mod usage;
pub(crate) use message::*;
pub(crate) use usage::*;
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotResponse {
    pub(crate) id: String,
    pub(crate) model: String,
    pub(crate) choices: Vec<MoonshotChoice>,
    #[serde(default)]
    pub(crate) usage: Option<MoonshotUsage>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotChoice {
    pub(crate) message: MoonshotMessage,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
