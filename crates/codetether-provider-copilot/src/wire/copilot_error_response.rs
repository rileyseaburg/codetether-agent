//! CopilotErrorResponse wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotErrorResponse {
    pub(crate) error: Option<CopilotErrorDetail>,
    pub(crate) message: Option<String>,
}
