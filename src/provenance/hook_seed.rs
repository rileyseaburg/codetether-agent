use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::{ExecutionProvenance, commit_editmsg_path, enrich_from_repo, provenance_trailers};

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
