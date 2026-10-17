//! Credential configuration operation: repo git path.
use super::*;

pub(super) fn repo_git_path(repo_path: &Path, path: &str) -> Result<PathBuf> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--git-path", path])
        .output()
        .with_context(|| format!("Failed to resolve Git path for {}", repo_path.display()))?;
    if !output.status.success() {
        return Err(anyhow!(
            "Git path resolution failed in {}: {}",
            repo_path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let resolved = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if resolved.is_empty() {
        return Err(anyhow!(
            "Git path resolution returned an empty path in {}",
            repo_path.display()
        ));
    }
    let path = PathBuf::from(resolved);
    Ok(if path.is_absolute() {
        path
    } else {
        repo_path.join(path)
    })
}
