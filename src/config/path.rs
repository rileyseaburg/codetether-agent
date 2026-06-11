use crate::config::Config;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

impl Config {
    /// Get the global config file path.
    pub fn global_config_path() -> Option<PathBuf> {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Get the data directory path.
    pub fn data_dir() -> Option<PathBuf> {
        if let Ok(explicit) = std::env::var("CODETETHER_DATA_DIR") {
            let explicit = explicit.trim();
            if !explicit.is_empty() {
                return Some(PathBuf::from(explicit));
            }
        }
        workspace_data_dir().or_else(|| {
            ProjectDirs::from("ai", "codetether", "codetether-agent")
                .map(|dirs| dirs.data_dir().to_path_buf())
        })
    }

    pub(crate) fn data_dir_for_workspace(workspace: &Path) -> Option<PathBuf> {
        if let Ok(explicit) = std::env::var("CODETETHER_DATA_DIR") {
            let explicit = explicit.trim();
            if !explicit.is_empty() {
                return Some(PathBuf::from(explicit));
            }
        }
        Some(workspace_data_dir_from(workspace))
    }
}

fn workspace_data_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    Some(workspace_data_dir_from(&cwd))
}

pub(super) fn workspace_data_dir_from(start: &Path) -> PathBuf {
    detect_workspace_root(start)
        .unwrap_or_else(|| start.to_path_buf())
        .join(".codetether-agent")
}

pub(super) fn detect_workspace_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|path| path.join(".git").exists())
        .map(Path::to_path_buf)
}
