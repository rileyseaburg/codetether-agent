use crate::provenance::ExecutionProvenance;
use anyhow::{Context, Result};
use std::path::Path;

use super::{
    commit_args_with_signature_policy, enrich_from_repo, git_identity_env_vars, provenance_trailers,
};

pub fn commit_message_with_provenance(
    commit_msg: &str,
    provenance: Option<&ExecutionProvenance>,
) -> String {
    let Some(provenance) = provenance else {
        return commit_msg.to_string();
    };
    let trailers = build_trailers(commit_msg, provenance);
    if trailers.is_empty() {
        return commit_msg.to_string();
    }
    let trailer_lines = trailers
        .into_iter()
        .map(|(label, value)| format!("{label}: {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n\n{}", commit_msg.trim_end(), trailer_lines)
}

pub async fn git_commit_with_provenance(
    repo_path: &Path,
    commit_msg: &str,
    provenance: Option<&ExecutionProvenance>,
) -> Result<std::process::Output> {
    let enriched = enrich_repo_provenance(provenance, repo_path);
    let message = commit_message_with_provenance(commit_msg, enriched.as_ref());
    let mut command = tokio::process::Command::new("git");
    command
        .args(commit_args_with_signature_policy())
        .arg(&message)
        .current_dir(repo_path);
    apply_git_identity_env_tokio(&mut command, enriched.as_ref());
    command
        .output()
        .await
        .context("Failed to execute git commit")
}

pub fn git_commit_with_provenance_blocking(
    repo_path: &Path,
    commit_msg: &str,
    provenance: Option<&ExecutionProvenance>,
) -> Result<std::process::Output> {
    let enriched = enrich_repo_provenance(provenance, repo_path);
    let message = commit_message_with_provenance(commit_msg, enriched.as_ref());
    let mut command = std::process::Command::new("git");
    command
        .args(commit_args_with_signature_policy())
        .arg(&message)
        .current_dir(repo_path);
    apply_git_identity_env_blocking(&mut command, enriched.as_ref());
    command.output().context("Failed to execute git commit")
}

fn enrich_repo_provenance(
    provenance: Option<&ExecutionProvenance>,
    repo_path: &Path,
) -> Option<ExecutionProvenance> {
    provenance.map(|value| enrich_from_repo(value, repo_path))
}

fn build_trailers(
    commit_msg: &str,
    provenance: &ExecutionProvenance,
) -> Vec<(&'static str, String)> {
    provenance_trailers(provenance)
        .into_iter()
        .filter(|(label, _)| !commit_msg.contains(label))
        .collect()
}

fn apply_git_identity_env_tokio(
    command: &mut tokio::process::Command,
    provenance: Option<&ExecutionProvenance>,
) {
    for (key, value) in git_identity_env_vars(provenance) {
        command.env(key, value);
    }
}

fn apply_git_identity_env_blocking(
    command: &mut std::process::Command,
    provenance: Option<&ExecutionProvenance>,
) {
    for (key, value) in git_identity_env_vars(provenance) {
        command.env(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::commit_message_with_provenance;
    use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

    #[test]
    fn appends_codetether_trailers() {
        let mut provenance = ExecutionProvenance::for_operation("ralph", ExecutionOrigin::Ralph);
        provenance.session_id = Some("session-1".to_string());
        provenance.apply_worker_task("worker-1", "task-1");
        provenance.set_run_id("run-1");
        provenance.attempt_id = Some("run-1:attempt:2".to_string());
        let message = commit_message_with_provenance("feat: test", Some(&provenance));
        assert!(message.contains("CodeTether-Provenance-ID:"));
        assert!(message.contains("CodeTether-Session-ID: session-1"));
        assert!(message.contains("CodeTether-Task-ID: task-1"));
        assert!(message.contains("CodeTether-Run-ID: run-1"));
        assert!(message.contains("CodeTether-Attempt-ID: run-1:attempt:2"));
    }
}
