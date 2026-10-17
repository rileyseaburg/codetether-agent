//! Glm5StreamResponse wire representation.
use crate::*;
// ─── SSE stream types ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct Glm5StreamResponse {
    pub(crate) choices: Vec<Glm5StreamChoice>,
    #[serde(default)]
    pub(crate) usage: Option<Glm5Usage>,
}
