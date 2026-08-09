//! Loads proposed patch contents into an approval edit session.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};

use super::{ApprovalEditFile, ApprovalEditSession};

pub(super) fn session(root: &Path, id: String, patch: &str) -> Result<ApprovalEditSession> {
    let proposed = crate::tool::patch::proposed::contents(root, patch)?;
    ensure!(!proposed.is_empty(), "approval patch has no editable files");
    let files = proposed
        .into_iter()
        .map(|(path, revised)| file(root, path, revised))
        .collect::<Result<Vec<_>>>()?;
    Ok(ApprovalEditSession {
        id,
        files,
        index: 0,
    })
}

fn file(root: &Path, path: PathBuf, revised: String) -> Result<ApprovalEditFile> {
    let relative = path
        .strip_prefix(root)
        .context("patch path escaped workspace")?
        .display()
        .to_string();
    let original = std::fs::read_to_string(&path).unwrap_or_default();
    Ok(ApprovalEditFile {
        path,
        relative,
        original,
        revised,
    })
}
