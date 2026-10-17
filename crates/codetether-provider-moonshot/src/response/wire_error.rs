//! Provider error wire data.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotError {
    #[allow(dead_code)]
    pub(crate) error: MoonshotErrorDetail,
}
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotErrorDetail {
    pub(crate) message: String,
    #[serde(default, rename = "type")]
    pub(crate) error_type: Option<String>,
}
