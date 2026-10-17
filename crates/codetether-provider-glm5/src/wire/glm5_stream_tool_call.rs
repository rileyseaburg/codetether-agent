//! Glm5StreamToolCall wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5StreamToolCall {
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) function: Option<Glm5StreamFunction>,
}
