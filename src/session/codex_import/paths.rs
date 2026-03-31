use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn codex_home_dir() -> Option<PathBuf> {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".codex")))
}

pub(crate) fn codex_home_dir_for_path(path: &Path) -> Option<PathBuf> {
    let mut current = path.parent()?;
    loop {
        if current.file_name().and_then(|name| name.to_str()) == Some(".codex") {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

pub(crate) fn native_sessions_dir() -> Result<PathBuf> {
    crate::config::Config::data_dir()
        .map(|dir| dir.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

pub(crate) fn native_session_path(data_dir: &Path, id: &str) -> PathBuf {
    data_dir.join(format!("{id}.json"))
}

pub(crate) fn canonicalize_loose(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub(crate) fn is_jsonl(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("jsonl")
}
