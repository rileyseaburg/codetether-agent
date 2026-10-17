//! Glm5Response wire representation.
use crate::*;
// ─── Response deserialization ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct Glm5Response {
    pub(crate) choices: Vec<Glm5Choice>,
    #[serde(default)]
    pub(crate) usage: Option<Glm5Usage>,
}
