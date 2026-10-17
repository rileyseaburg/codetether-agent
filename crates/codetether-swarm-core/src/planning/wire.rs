//! Serde wire representation of a model-authored task list.
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct DecompositionResponse {
    pub(super) subtasks: Vec<SubTaskDef>,
}

#[derive(Deserialize)]
pub(super) struct SubTaskDef {
    pub(super) name: String,
    pub(super) instruction: String,
    pub(super) specialty: Option<String>,
    #[serde(default)]
    pub(super) dependencies: Vec<String>,
    #[serde(default)]
    pub(super) priority: i32,
    #[serde(default)]
    pub(super) needs_worktree: Option<bool>,
}
