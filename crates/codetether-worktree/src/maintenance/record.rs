use std::path::PathBuf;

#[derive(Debug, Default)]
pub(super) struct WorktreeRecord {
    pub(super) path: PathBuf,
    pub(super) head: String,
    pub(super) branch: Option<String>,
    pub(super) primary: bool,
    pub(super) locked: bool,
    pub(super) prunable: bool,
}
