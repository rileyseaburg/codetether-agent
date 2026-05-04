use super::Session;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

/// Summary of a session for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub agent: String,
    /// The working directory this session was created in
    #[serde(default)]
    pub directory: Option<PathBuf>,
}

/// List all sessions
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    let sessions_dir = crate::config::Config::data_dir()
        .map(|d| d.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;

    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let content = match fs::read_to_string(&path).await {
                Ok(c) => c,
                Err(err) => {
                    tracing::warn!(path = %path.display(), error = %err, "skipping unreadable session file");
                    continue;
                }
            };
            let session = match serde_json::from_str::<Session>(&content) {
                Ok(s) => s,
                Err(err) => {
                    tracing::warn!(path = %path.display(), error = %err, "skipping malformed session file");
                    continue;
                }
            };
            summaries.push(SessionSummary {
                id: session.id,
                title: session.title,
                created_at: session.created_at,
                updated_at: session.updated_at,
                message_count: session.messages.len(),
                agent: session.agent,
                directory: session.metadata.directory,
            });
        }
    }

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

/// List sessions scoped to a specific directory (workspace)
///
/// Only returns sessions whose `metadata.directory` matches the given path.
/// This prevents sessions from other workspaces "leaking" into the TUI.
pub async fn list_sessions_for_directory(dir: &std::path::Path) -> Result<Vec<SessionSummary>> {
    let all = list_sessions().await?;
    let canonical = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    Ok(all
        .into_iter()
        .filter(|s| {
            s.directory
                .as_ref()
                .map(|d| {
                    match d.canonicalize() {
                        Ok(c) => c == canonical,
                        Err(_) => {
                            // Fallback: prefix match if either side can't canonicalize
                            d == dir || canonical.starts_with(d) || d.starts_with(&canonical)
                        }
                    }
                })
                .unwrap_or(false)
        })
        .collect())
}

/// List sessions for a directory with pagination.
///
/// - `limit`: Maximum number of sessions to return (default: 100)
/// - `offset`: Number of sessions to skip (default: 0)
pub async fn list_sessions_paged(
    dir: &std::path::Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    let mut sessions = list_sessions_for_directory(dir).await?;
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions.into_iter().skip(offset).take(limit).collect())
}
