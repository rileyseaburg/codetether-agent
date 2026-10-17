//! Error deserialization for DeepSeek API responses.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DsError {
    pub error: DsErrorDetail,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsErrorDetail {
    pub message: String,
    #[serde(default, rename = "type")]
    pub error_type: Option<String>,
}
