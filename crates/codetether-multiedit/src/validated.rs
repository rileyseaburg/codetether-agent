//! Accumulated edits and cached final file content.
use std::{collections::HashMap, path::PathBuf};
#[derive(Default)]
pub(super) struct Validated {
    pub edits: Vec<(PathBuf, String, String)>,
    pub cache: HashMap<PathBuf, String>,
    pub errors: Vec<String>,
}
