//! Glm5StreamFunction wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5StreamFunction {
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) arguments: Option<Value>,
}
