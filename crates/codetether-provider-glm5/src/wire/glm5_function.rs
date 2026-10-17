//! Glm5Function wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5Function {
    pub(crate) name: String,
    pub(crate) arguments: Value,
}
