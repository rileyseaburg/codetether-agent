//! error wire schema.
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(crate) struct ErrorResponse {
    pub(crate) error: ErrorDetail,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ErrorDetail {
    pub(crate) message: String,
    #[serde(default)]
    pub(crate) code: Option<String>,
}
