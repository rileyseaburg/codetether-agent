//! Concurrent lookup and ownership of running command sessions.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;

use super::Running;

const MAX_SESSIONS: usize = 64;

/// Owns up to 64 running commands, identified by monotonically assigned IDs.
///
/// # Examples
/// ```rust
/// let _registry = codetether_command_session::Registry::default();
/// ```
#[derive(Default)]
pub struct Registry {
    next_id: AtomicU64,
    sessions: Mutex<HashMap<u64, Arc<Mutex<Running>>>>,
}

impl Registry {
    /// Retain a command and assign its session ID.
    ///
    /// # Arguments
    /// * `command` - Running command whose lifetime the registry will own.
    /// # Returns
    /// The newly assigned ID.
    /// # Errors
    /// Returns an error when 64 sessions are already retained.
    pub async fn insert(&self, command: Running) -> Result<u64> {
        let mut sessions = self.sessions.lock().await;
        if sessions.len() >= MAX_SESSIONS {
            return Err(anyhow!(
                "at most {MAX_SESSIONS} command sessions may run at once"
            ));
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        sessions.insert(id, Arc::new(Mutex::new(command)));
        Ok(id)
    }

    /// Clone the shared command handle for `id`, or return `None`.
    pub async fn get(&self, id: u64) -> Option<Arc<Mutex<Running>>> {
        self.sessions.lock().await.get(&id).cloned()
    }

    /// Remove registry ownership of `id`; remaining shared handles stay valid.
    pub async fn remove(&self, id: u64) {
        self.sessions.lock().await.remove(&id);
    }
}
