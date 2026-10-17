//! CopilotModelCapabilities wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelCapabilities {
    #[serde(default)]
    pub(crate) limits: Option<CopilotModelLimits>,
    #[serde(default)]
    pub(crate) supports: Option<CopilotModelSupports>,
}
