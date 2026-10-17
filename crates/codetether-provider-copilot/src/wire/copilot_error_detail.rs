//! CopilotErrorDetail wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct CopilotErrorDetail {
    pub(crate) message: Option<String>,
    pub(crate) code: Option<String>,
}
