use std::path::Path;

pub(super) async fn registered(repo: &Path, path: &Path, force: bool) -> Result<(), String> {
    let mut command = tokio::process::Command::new("git");
    command.args(["worktree", "remove"]);
    if force {
        command.arg("--force");
    }
    let output = command
        .arg("--")
        .arg(path)
        .current_dir(repo)
        .output()
        .await
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    }
}
