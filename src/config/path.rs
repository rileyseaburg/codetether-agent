use crate::config::Config;
use std::path::{Path, PathBuf};

impl Config {
    /// Get the global config file path.
    pub fn global_config_path() -> Option<PathBuf> {
        codetether_paths::project_dirs().map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Get the data directory path. See [`codetether_paths::data_dir`].
    pub fn data_dir() -> Option<PathBuf> {
        codetether_paths::data_dir()
    }

    pub(crate) fn data_dir_for_workspace(workspace: &Path) -> Option<PathBuf> {
        codetether_paths::data_dir_for_workspace(workspace)
    }
}

pub(super) use codetether_paths::detect_workspace_root;
