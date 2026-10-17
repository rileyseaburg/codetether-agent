//! CopilotModelInfo wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelInfo {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) model_picker_enabled: Option<bool>,
    #[serde(default)]
    pub(crate) policy: Option<CopilotModelPolicy>,
    #[serde(default)]
    pub(crate) capabilities: Option<CopilotModelCapabilities>,
}
