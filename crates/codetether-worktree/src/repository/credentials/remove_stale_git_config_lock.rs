//! Credential configuration operation: remove stale git config lock.
use super::*;

pub(super) fn remove_stale_git_config_lock(repo_path: &Path, stale_after: Duration) -> Result<()> {
    let lock_path = repo_git_path(repo_path, "config.lock")?;
    let metadata = std::fs::metadata(&lock_path).with_context(|| {
        format!(
            "Git reported a config lock, but {} could not be inspected",
            lock_path.display()
        )
    })?;
    let modified = metadata.modified().with_context(|| {
        format!(
            "Failed to read modification time for Git config lock {}",
            lock_path.display()
        )
    })?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or_default();
    if age < stale_after {
        return Err(anyhow!(
            "Git config lock {} is too new to remove safely (age {:?}, threshold {:?})",
            lock_path.display(),
            age,
            stale_after
        ));
    }
    std::fs::remove_file(&lock_path).with_context(|| {
        format!(
            "Failed to remove stale Git config lock {}",
            lock_path.display()
        )
    })?;
    tracing::warn!(target: "codetether_agent::a2a::git_credentials::repo_config",
        repo_path = %repo_path.display(),
        lock_path = %lock_path.display(),
        age_ms = age.as_millis(),
        "Removed stale Git config lock before retrying git config"
    );
    Ok(())
}
