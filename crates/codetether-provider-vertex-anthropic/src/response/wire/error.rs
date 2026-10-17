//! error wire data.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicError {
    pub(crate) error: AnthropicErrorDetail,
}
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicErrorDetail {
    pub(crate) message: String,
    #[serde(default, rename = "type")]
    pub(crate) error_type: Option<String>,
}
