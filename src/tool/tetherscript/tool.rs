use std::path::{Path, PathBuf};

pub struct TetherScriptPluginTool {
    root: PathBuf,
}

impl TetherScriptPluginTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

impl Default for TetherScriptPluginTool {
    fn default() -> Self {
        Self::new()
    }
}
