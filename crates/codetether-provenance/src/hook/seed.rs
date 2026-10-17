//! Attach missing provenance trailers to the pending commit message.
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::paths::commit_editmsg_path;
use crate::{ExecutionProvenance, enrich_from_repo, provenance_trailers};

/// Add missing provenance trailers to the pending commit message.
///
/// # Arguments
/// * `repo_path` - Repository containing the pending commit message.
/// * `provenance` - Record enriched with repository identity before trailer emission.
///
/// # Returns
/// Unit after each trailer has been processed.
///
/// # Errors
/// Returns an error if the message cannot be initialized or Git trailer processing fails.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::{ExecutionProvenance, ensure_provenance_trailers};
/// let provenance = ExecutionProvenance::for_session("session", "worker");
/// ensure_provenance_trailers(std::path::Path::new("."), &provenance)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn ensure_provenance_trailers(
    repo_path: &Path,
    provenance: &ExecutionProvenance,
) -> Result<()> {
    let editmsg = commit_editmsg_path(repo_path)?;
    if !editmsg.exists() {
        fs::write(&editmsg, "").context("Failed to initialize COMMIT_EDITMSG")?;
    }
    let enriched = enrich_from_repo(provenance, repo_path);
    for (label, value) in provenance_trailers(&enriched) {
        add_trailer(repo_path, &editmsg, label, &value)?;
    }
    Ok(())
}

fn add_trailer(repo_path: &Path, editmsg: &Path, label: &str, value: &str) -> Result<()> {
    let trailer = format!("{label}: {value}");
    let status = Command::new("git")
        .args([
            "interpret-trailers",
            "--in-place",
            "--if-exists",
            "doNothing",
        ])
        .arg("--trailer")
        .arg(&trailer)
        .arg(editmsg)
        .current_dir(repo_path)
        .status()
        .with_context(|| format!("Failed to add provenance trailer {label}"))?;
    if !status.success() {
        anyhow::bail!("git interpret-trailers failed while adding {label}");
    }
    Ok(())
}
