//! Concurrent lookup and ownership of running command sessions.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;

use super::Running;

const MAX_SESSIONS: usize = 64;

#[derive(Default)]
pub(crate) struct Registry {
    next_id: AtomicU64,
    sessions: Mutex<HashMap<u64, Arc<Mutex<Running>>>>,
}

impl Registry {
    pub(crate) async fn insert(&self, command: Running) -> Result<u64> {
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

    pub(crate) async fn get(&self, id: u64) -> Option<Arc<Mutex<Running>>> {
        self.sessions.lock().await.get(&id).cloned()
    }

    pub(crate) async fn remove(&self, id: u64) {
        self.sessions.lock().await.remove(&id);
    }
}
