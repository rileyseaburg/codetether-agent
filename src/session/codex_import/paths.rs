//! Native storage paths and shared Codex archive-home resolution.
use anyhow::Result;
pub(crate) use codetether_codex_discovery::{codex_home_dir, codex_home_dir_for_path};
use std::path::{Path, PathBuf};

pub(crate) fn native_sessions_dir() -> Result<PathBuf> {
    crate::config::Config::data_dir()
        .map(|dir| dir.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

pub(crate) fn native_session_path(data_dir: &Path, id: &str) -> PathBuf {
    data_dir.join(format!("{id}.json"))
}
