//! Provider error envelope.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ApiError {
    pub(crate) error: ApiErrorDetail,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorDetail {
    pub(crate) message: String,
}
