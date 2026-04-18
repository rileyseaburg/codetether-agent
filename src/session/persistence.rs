//! On-disk persistence: save, load, delete, and directory lookup.

use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;

use super::types::Session;

impl Session {
    /// Load an existing session by its UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the session file does not exist or the JSON is
    /// malformed.
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let content = fs::read_to_string(&path).await?;
        let session: Session = serde_json::from_str(&content)?;
        Ok(session)
    }

    /// Load the most recent session, optionally scoped to a workspace
    /// directory.
    ///
    /// When `workspace` is [`Some`], only considers sessions created in that
    /// directory. When [`None`], returns the most recent session globally.
    ///
    /// # Errors
    ///
    /// Returns an error if no sessions exist (or, with `workspace` set,
    /// none match the requested directory).
    pub async fn last_for_directory(workspace: Option<&std::path::Path>) -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;
        if !sessions_dir.exists() {
            anyhow::bail!("No sessions found");
        }

        let mut entries: Vec<tokio::fs::DirEntry> = Vec::new();
        let mut read_dir = fs::read_dir(&sessions_dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            entries.push(entry);
        }
        if entries.is_empty() {
            anyhow::bail!("No sessions found");
        }

        // Sort by modification time, most recent first.
        entries.sort_by_key(|e| {
            std::cmp::Reverse(
                std::fs::metadata(e.path())
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            )
        });

        let canonical_workspace =
            workspace.map(|w| w.canonicalize().unwrap_or_else(|_| w.to_path_buf()));

        for entry in &entries {
            let content = match fs::read_to_string(entry.path()).await {
                Ok(c) => c,
                Err(err) => {
                    tracing::warn!(
                        path = %entry.path().display(),
                        error = %err,
                        "skipping unreadable session file",
                    );
                    continue;
                }
            };
            if let Ok(session) = serde_json::from_str::<Session>(&content) {
                if let Some(ref ws) = canonical_workspace {
                    if let Some(ref dir) = session.metadata.directory {
                        let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
                        if &canonical_dir == ws {
                            return Ok(session);
                        }
                    }
                    continue;
                }
                return Ok(session);
            }
        }

        anyhow::bail!("No sessions found")
    }

    /// Load the most recent session globally (unscoped).
    ///
    /// Kept for legacy compatibility; prefer
    /// [`Session::last_for_directory`].
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Persist the session to disk as JSON. Creates the sessions directory
    /// on demand.
    pub async fn save(&self) -> Result<()> {
        let path = Self::session_path(&self.id)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        Ok(())
    }

    /// Delete a session file by ID. No-op if the file does not exist.
    pub async fn delete(id: &str) -> Result<()> {
        let path = Self::session_path(id)?;
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Resolve the sessions directory (`<data_dir>/sessions`).
    pub(crate) fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|d| d.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }

    /// Resolve the on-disk path for a session file.
    pub(crate) fn session_path(id: &str) -> Result<PathBuf> {
        Ok(Self::sessions_dir()?.join(format!("{}.json", id)))
    }
}
