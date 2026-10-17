//! Glm5Error wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5Error {
    pub(crate) error: Glm5ErrorDetail,
}
