//! params for the planning tools.
use super::*;

#[derive(Deserialize)]
pub(crate) struct EnterParams {
    pub(crate) goal: String,
    pub(crate) steps: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct ExitParams {
    #[serde(default)]
    pub(crate) summary: Option<String>,
    #[serde(default)]
    pub(crate) step_complete: Option<usize>,
    #[serde(default)]
    pub(crate) notes: Option<String>,
}
