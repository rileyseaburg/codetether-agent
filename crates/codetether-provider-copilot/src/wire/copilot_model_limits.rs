//! CopilotModelLimits wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelLimits {
    #[serde(default)]
    pub(crate) max_context_window_tokens: Option<usize>,
    #[serde(default)]
    pub(crate) max_output_tokens: Option<usize>,
}
