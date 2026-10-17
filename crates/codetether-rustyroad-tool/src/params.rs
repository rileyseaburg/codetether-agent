//! Typed first-party action inputs.

use super::environment::Environment;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Action {
    ListTools,
    CallTool,
}

#[derive(Debug, Deserialize)]
pub(super) struct Params {
    pub(super) action: Action,
    pub(super) cwd: PathBuf,
    #[serde(default)]
    pub(super) environment: Environment,
    pub(super) tool_name: Option<String>,
    #[serde(default)]
    pub(super) arguments: Map<String, Value>,
}
