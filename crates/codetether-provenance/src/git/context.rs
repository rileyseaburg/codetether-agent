//! Repository provenance enrichment and Git environment application.

use crate::{ExecutionProvenance, enrich_from_repo, git_identity_env_vars};
use std::path::Path;

pub(super) fn enrich_repo_provenance(
    provenance: Option<&ExecutionProvenance>,
    repo_path: &Path,
) -> Option<ExecutionProvenance> {
    provenance.map(|value| enrich_from_repo(value, repo_path))
}

pub(super) fn apply_git_identity_env_tokio(
    command: &mut tokio::process::Command,
    provenance: Option<&ExecutionProvenance>,
) {
    for (key, value) in git_identity_env_vars(provenance) {
        command.env(key, value);
    }
}

pub(super) fn apply_git_identity_env_blocking(
    command: &mut std::process::Command,
    provenance: Option<&ExecutionProvenance>,
) {
    for (key, value) in git_identity_env_vars(provenance) {
        command.env(key, value);
    }
}
