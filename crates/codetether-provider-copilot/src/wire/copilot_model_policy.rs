//! CopilotModelPolicy wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelPolicy {
    #[serde(default)]
    pub(crate) state: Option<String>,
}
