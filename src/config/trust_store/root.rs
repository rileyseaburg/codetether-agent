use crate::config::Config;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

pub(super) fn trust_root_for(workspace: &Path) -> Option<PathBuf> {
    let data_dir = Config::data_dir_for_workspace(workspace)?;
    let base = if is_inside_workspace(&data_dir, workspace) {
        global_data_dir()?
    } else {
        data_dir
    };
    Some(base.join("trust-store").join("projects"))
}

fn global_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("ai", "codetether", "codetether-agent")
        .map(|dirs| dirs.data_dir().to_path_buf())
}

fn is_inside_workspace(path: &Path, workspace: &Path) -> bool {
    absolute(path).starts_with(absolute(workspace))
}

fn absolute(path: &Path) -> PathBuf {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };
    std::fs::canonicalize(&path).unwrap_or(path)
}
