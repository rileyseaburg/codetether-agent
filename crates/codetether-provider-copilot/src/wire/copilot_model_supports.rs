//! CopilotModelSupports wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelSupports {
    #[serde(default)]
    pub(crate) tool_calls: Option<bool>,
    #[serde(default)]
    pub(crate) vision: Option<bool>,
    #[serde(default)]
    pub(crate) streaming: Option<bool>,
}
