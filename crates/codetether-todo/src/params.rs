//! params for session todo tools.
use super::*;

#[derive(Deserialize)]
pub(crate) struct ReadParams {
    #[serde(default)]
    pub(crate) status: Option<String>,
    #[serde(default)]
    pub(crate) priority: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct WriteParams {
    pub(crate) action: String, // add, update, delete, clear
    #[serde(default)]
    pub(crate) id: Option<String>,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) status: Option<String>,
    #[serde(default)]
    pub(crate) priority: Option<String>,
}
