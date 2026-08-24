pub(super) fn is_git_repo() -> bool {
    std::path::Path::new(".git").exists() || git_success(&["rev-parse", "--git-dir"])
}

fn git_output(args: &[&str]) -> anyhow::Result<std::process::Output> {
    let cwd = std::env::current_dir()?;
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    crate::tool::git::process::output_blocking(&cwd, &args, &[], false)
}

pub(super) fn current_branch() -> Option<String> {
    git_stdout(&["rev-parse", "--abbrev-ref", "HEAD"]).map(|branch| branch.trim().to_string())
}

pub(super) fn status_porcelain() -> String {
    git_stdout(&["status", "--porcelain"]).unwrap_or_default()
}

pub(super) fn can_create_worktrees() -> bool {
    git_success(&["worktree", "list"])
}

fn git_success(args: &[&str]) -> bool {
    git_output(args)
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn git_stdout(args: &[&str]) -> Option<String> {
    git_output(args)
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
}
