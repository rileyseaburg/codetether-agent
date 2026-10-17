//! Required hooks declared by repository files.
use std::path::Path;

/// Hook names each declaration requires in the active hooks directory.
pub(super) fn required_hooks(repo_path: &Path) -> Vec<&'static str> {
    let mut hooks = Vec::new();
    if repo_path.join(".pre-commit-config.yaml").is_file() {
        hooks.push("pre-commit");
        if declares_post_commit(repo_path) {
            hooks.push("post-commit");
        }
    }
    if repo_path.join(".githooks/pre-push").is_file() {
        hooks.push("pre-push");
    }
    hooks
}

fn declares_post_commit(repo_path: &Path) -> bool {
    std::fs::read_to_string(repo_path.join(".pre-commit-config.yaml"))
        .map(|text| text.contains("post-commit"))
        .unwrap_or(true)
}
