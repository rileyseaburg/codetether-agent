//! Glm5Choice wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5Choice {
    pub(crate) message: Glm5Message,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
