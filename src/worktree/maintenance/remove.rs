use std::path::Path;

pub(super) async fn registered(repo: &Path, path: &Path, force: bool) -> Result<(), String> {
    let mut args = vec!["worktree".into(), "remove".into()];
    if force {
        args.push("--force".into());
    }
    args.extend(["--".into(), path.display().to_string()]);
    let output = crate::tool::git::process::output(repo, &args, &[], true)
        .await
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    }
}
