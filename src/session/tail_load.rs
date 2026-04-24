//! Bounded-memory session load using the tail-cap deserializer.
//!
//! `Session::load_tail` opens the session file with an incremental JSON
//! reader and keeps only the last `window` messages/tool uses, so resuming
//! a very large session does not allocate the full transcript.

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::tail_seed::with_tail_cap;
use super::types::Session;

/// Result of a bounded session load.
#[derive(Debug)]
pub struct TailLoad {
    /// The session with its message/tool_use vectors capped to the window.
    pub session: Session,
    /// Total number of messages + tool_use entries discarded from the tail
    /// cap. Zero means the full transcript fit within the window.
    pub dropped: usize,
    /// Size of the session file on disk, in bytes.
    pub file_bytes: u64,
}

impl Session {
    /// Load a session by UUID, keeping only the last `window` messages and
    /// tool uses. Use `window = 0` to drop all messages (cheap directory
    /// scan).
    ///
    /// # Errors
    /// Returns an error if the file cannot be opened or the JSON is malformed.
    pub async fn load_tail(id: &str, window: usize) -> Result<TailLoad> {
        let path = Self::session_path(id)?;
        Self::load_tail_from_path(path, window).await
    }

    /// Load a session from an explicit path with a bounded message window.
    pub async fn load_tail_from_path(path: PathBuf, window: usize) -> Result<TailLoad> {
        tokio::task::spawn_blocking(move || parse_tail(&path, window))
            .await
            .context("session tail-load task panicked")?
    }
}

fn parse_tail(path: &Path, window: usize) -> Result<TailLoad> {
    let file_bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let file = File::open(path).with_context(|| format!("open session file {}", path.display()))?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    let (parsed, dropped) = with_tail_cap(window, || {
        serde_json::from_reader::<_, Session>(reader)
            .with_context(|| format!("parse session file {}", path.display()))
    });
    let mut session = parsed?;
    session.normalize_sidecars();
    Ok(TailLoad {
        session,
        dropped,
        file_bytes,
    })
}
