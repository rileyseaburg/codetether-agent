/// Host-wide guard that serializes OAuth refreshes for one provider.
///
/// Every CodeTether process on the machine (for example, every agent under one
/// mux server) opens the same lock file, so only one of them spends the
/// single-use refresh token at a time. The OS releases the lock on drop.
struct RefreshLock {
    _file: std::fs::File,
}

impl RefreshLock {
    /// Acquire the host-wide lock, or log and continue unlocked on failure.
    async fn acquire_for(provider_id: &str) -> Option<Self> {
        let path = refresh_lock_dir().join(format!("{provider_id}-oauth-refresh.lock"));
        match Self::acquire_at(&path, std::time::Duration::from_secs(90)).await {
            Ok(lock) => Some(lock),
            Err(error) => {
                tracing::warn!(path = %path.display(), %error, "Codex refresh lock unavailable");
                None
            }
        }
    }

    async fn acquire_at(path: &std::path::Path, timeout: std::time::Duration) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).context("Failed to create refresh lock directory")?;
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .context("Failed to open refresh lock file")?;
        let deadline = std::time::Instant::now() + timeout;
        loop {
            match file.try_lock() {
                Ok(()) => return Ok(Self { _file: file }),
                Err(std::fs::TryLockError::WouldBlock) if std::time::Instant::now() < deadline => {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Err(std::fs::TryLockError::WouldBlock) => anyhow::bail!("Timed out waiting"),
                Err(std::fs::TryLockError::Error(error)) => return Err(error.into()),
            }
        }
    }
}

fn refresh_lock_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("ai", "codetether", "codetether-agent")
        .map(|dirs| dirs.data_local_dir().join("locks"))
        .unwrap_or_else(std::env::temp_dir)
}
