use std::path::{Path, PathBuf};

pub struct KilnPluginTool {
    root: PathBuf,
}

impl KilnPluginTool {
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

impl Default for KilnPluginTool {
    fn default() -> Self {
        Self::new()
    }
}
