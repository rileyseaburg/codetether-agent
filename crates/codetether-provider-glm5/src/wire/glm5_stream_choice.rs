//! Glm5StreamChoice wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5StreamChoice {
    pub(crate) delta: Glm5StreamDelta,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
