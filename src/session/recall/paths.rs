//! Stable paths for recall sidecars and workspace catalogs.

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use crate::session::Session;

pub(crate) fn canonical(workspace: &Path) -> PathBuf {
    workspace
        .canonicalize()
        .unwrap_or_else(|_| workspace.to_path_buf())
}

pub(crate) fn session(session_id: &str) -> Result<PathBuf> {
    Session::session_path(session_id)?;
    Ok(root()?.join("sessions").join(format!("{session_id}.json")))
}

pub(crate) fn catalog(workspace: &Path) -> Result<PathBuf> {
    let workspace = canonical(workspace);
    let digest = Sha256::digest(workspace.to_string_lossy().as_bytes());
    Ok(root()?
        .join("workspaces")
        .join(format!("{}.json", hex::encode(digest))))
}

fn root() -> Result<PathBuf> {
    Ok(Session::sessions_dir()?.join("recall"))
}
