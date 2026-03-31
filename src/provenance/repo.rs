use super::ExecutionProvenance;
use std::path::Path;
use std::process::Command;

pub fn enrich_from_repo(provenance: &ExecutionProvenance, repo_path: &Path) -> ExecutionProvenance {
    let mut enriched = provenance.clone();
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
