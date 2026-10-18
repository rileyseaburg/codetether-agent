//! Serialized per-workspace budget configuration.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct FileConfig {
    #[serde(default)]
    pub(super) rules: Vec<FileRule>,
}

#[derive(Deserialize)]
pub(super) struct FileRule {
    pub(super) glob: String,
    pub(super) max_lines: usize,
}
