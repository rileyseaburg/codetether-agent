use super::ExecutionProvenance;
use std::path::Path;

pub fn enrich_from_repo(provenance: &ExecutionProvenance, repo_path: &Path) -> ExecutionProvenance {
    let mut enriched = provenance.clone();
    fill(
        &mut enriched.identity.agent_identity_id,
        super::runtime_agent_identity(),
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
    let args = ["config", "--local", "--get", key];
    let output = crate::tool::git::process::output_blocking_refs(repo_path, &args, false).ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
