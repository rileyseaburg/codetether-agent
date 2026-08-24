use std::path::Path;

pub(super) fn same_path(left: &Path, right: &Path) -> bool {
    normalized(left) == normalized(right)
}

pub(super) async fn dirty(path: &Path) -> Result<bool, String> {
    let output = crate::tool::git::process::output_refs(path, &["status", "--porcelain"], false)
        .await
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(!output.stdout.is_empty())
}

pub(super) async fn merged(repo: &Path, head: &str, base: &str) -> Result<bool, String> {
    let status = crate::tool::git::process::output_refs(
        repo,
        &["merge-base", "--is-ancestor", head, base],
        false,
    )
    .await
    .map_err(|error| error.to_string())?;
    match status.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err("git merge-base failed".into()),
    }
}

fn normalized(path: &Path) -> std::path::PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.into())
}
