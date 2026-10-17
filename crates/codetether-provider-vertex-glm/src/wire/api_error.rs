//! Wire representation of ApiError.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ApiError {
    pub(crate) error: ApiErrorDetail,
}
