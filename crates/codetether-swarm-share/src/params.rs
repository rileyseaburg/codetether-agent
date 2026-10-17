//! params for shared swarm results.
use super::*;

#[derive(Deserialize)]
pub(super) struct Params {
    pub(super) action: String,
    #[serde(default)]
    pub(super) key: Option<String>,
    #[serde(default)]
    pub(super) value: Option<Value>,
    #[serde(default)]
    pub(super) tags: Option<Vec<String>>,
    #[serde(default)]
    pub(super) prefix: Option<String>,
}
