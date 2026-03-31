use crate::provenance::ExecutionProvenance;
use anyhow::{Context, Result};
use std::path::Path;

use super::identity::{git_author_email, git_author_name};
use super::{enrich_from_repo, sign_provenance};

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
    format!("{}\n\n{}", commit_msg.trim_end(), trailers.join("\n"))
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
        .args(["commit", "-m", &message])
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
        .args(["commit", "-m", &message])
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

fn build_trailers(commit_msg: &str, provenance: &ExecutionProvenance) -> Vec<String> {
    trailer_pairs(provenance)
        .into_iter()
        .filter(|(label, _)| !commit_msg.contains(label))
        .map(|(label, value)| format!("{label}: {value}"))
        .collect()
}

fn trailer_pairs(provenance: &ExecutionProvenance) -> Vec<(&'static str, String)> {
    let mut trailers = vec![
        ("CodeTether-Provenance-ID", provenance.provenance_id.clone()),
        (
            "CodeTether-Origin",
            provenance.identity.origin.as_str().to_string(),
        ),
        (
            "CodeTether-Agent-Name",
            provenance.identity.agent_name.clone(),
        ),
    ];
    push_opt(
        &mut trailers,
        "CodeTether-Agent-Identity",
        provenance.identity.agent_identity_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Tenant-ID",
        provenance.identity.tenant_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Worker-ID",
        provenance.identity.worker_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Session-ID",
        provenance.session_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Task-ID",
        provenance.task_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Run-ID",
        provenance.run_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Attempt-ID",
        provenance.attempt_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Key-ID",
        provenance.identity.key_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-GitHub-Installation-ID",
        provenance.identity.github_installation_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-GitHub-App-ID",
        provenance.identity.github_app_id.clone(),
    );
    push_opt(
        &mut trailers,
        "CodeTether-Signature",
        sign_provenance(provenance),
    );
    trailers
}

fn push_opt(
    trailers: &mut Vec<(&'static str, String)>,
    label: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        trailers.push((label, value));
    }
}

fn git_identity_env_vars(provenance: Option<&ExecutionProvenance>) -> Vec<(&'static str, String)> {
    let Some(provenance) = provenance else {
        return Vec::new();
    };
    let name = git_author_name(provenance.identity.agent_identity_id.as_deref());
    let email = git_author_email(
        provenance.identity.agent_identity_id.as_deref(),
        provenance.identity.worker_id.as_deref(),
        Some(provenance.provenance_id.as_str()),
    );
    vec![
        ("GIT_AUTHOR_NAME", name.clone()),
        ("GIT_COMMITTER_NAME", name),
        ("GIT_AUTHOR_EMAIL", email.clone()),
        ("GIT_COMMITTER_EMAIL", email),
    ]
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
    use super::{commit_message_with_provenance, git_identity_env_vars};
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

    #[test]
    fn derives_real_git_identity_env() {
        let mut provenance = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
        provenance.identity.agent_identity_id = Some("user:abc-123".to_string());
        provenance.identity.worker_id = Some("wrk-1".to_string());
        let env = git_identity_env_vars(Some(&provenance));
        assert!(
            env.iter()
                .any(|(k, v)| *k == "GIT_AUTHOR_NAME" && v.contains("user-abc-123"))
        );
        assert!(
            env.iter()
                .any(|(k, v)| *k == "GIT_AUTHOR_EMAIL" && v == "agent+user-abc-123@codetether.run")
        );
    }
}
