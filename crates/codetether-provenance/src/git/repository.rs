//! Enrich execution metadata with local repository configuration.
use crate::ExecutionProvenance;
use std::path::Path;
use std::process::Command;

/// Fill missing identity metadata from runtime identity and repository configuration.
///
/// # Arguments
/// * `provenance` - Record cloned before filling missing fields.
/// * `repo_path` - Repository whose local Git configuration is inspected.
///
/// # Returns
/// A cloned record; existing identity fields take precedence.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::{ExecutionProvenance, enrich_from_repo};
/// let original = ExecutionProvenance::for_session("session", "worker");
/// let enriched = enrich_from_repo(&original, std::path::Path::new("."));
/// assert_eq!(enriched.provenance_id, original.provenance_id);
/// ```
pub fn enrich_from_repo(provenance: &ExecutionProvenance, repo_path: &Path) -> ExecutionProvenance {
    let mut enriched = provenance.clone();
    fill(
        &mut enriched.identity.agent_identity_id,
        crate::runtime_agent_identity(),
    );
    fill(
        &mut enriched.identity.github_installation_id,
        git_config(repo_path, "codetether.githubInstallationId"),
    );
    fill(
        &mut enriched.identity.github_app_id,
        git_config(repo_path, "codetether.githubAppId"),
    );
    enriched
}

fn fill(slot: &mut Option<String>, value: Option<String>) {
    if slot.is_none() {
        *slot = value;
    }
}

fn git_config(repo_path: &Path, key: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--local", "--get", key])
        .current_dir(repo_path)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
