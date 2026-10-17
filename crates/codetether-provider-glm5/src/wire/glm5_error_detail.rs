//! Glm5ErrorDetail wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5ErrorDetail {
    pub(crate) message: String,
    #[serde(default, rename = "type")]
    pub(crate) error_type: Option<String>,
}
