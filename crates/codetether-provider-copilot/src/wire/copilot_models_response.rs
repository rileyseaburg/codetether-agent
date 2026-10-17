//! CopilotModelsResponse wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotModelsResponse {
    pub(crate) data: Vec<CopilotModelInfo>,
}
