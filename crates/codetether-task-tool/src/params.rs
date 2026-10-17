//! params for in-memory task tracking.
use super::*;

#[derive(Deserialize)]
pub(super) struct Params {
    pub(super) action: String, // create, status, complete, list, cancel
    #[serde(default)]
    pub(super) id: Option<String>,
    #[serde(default)]
    pub(super) description: Option<String>,
    #[serde(default)]
    pub(super) result: Option<String>,
}
