//! Wire representation of ApiErrorDetail.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorDetail {
    pub(crate) message: String,
    #[serde(default, rename = "type")]
    pub(crate) error_type: Option<String>,
}
