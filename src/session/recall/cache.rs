//! Process-local cache of decoded workspace recall sidecars.

use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use super::indexed_session::IndexedSession;

type Sessions = Arc<Vec<IndexedSession>>;

pub(super) fn get(key: &Path) -> Option<Sessions> {
    entries().get(key).map(|entry| Arc::clone(entry.value()))
}

pub(super) fn insert(key: PathBuf, sessions: Vec<IndexedSession>) -> Sessions {
    let sessions = Arc::new(sessions);
    entries().insert(key, Arc::clone(&sessions));
    sessions
}

pub(super) fn update(key: &Path, session: IndexedSession) {
    let Some(current) = get(key) else {
        return;
    };
    let mut sessions = current.as_ref().clone();
    sessions.retain(|item| item.session_id != session.session_id);
    sessions.push(session);
    insert(key.to_path_buf(), sessions);
}

pub(super) fn remove(key: &Path, session_id: &str) {
    let Some(current) = get(key) else {
        return;
    };
    let mut sessions = current.as_ref().clone();
    sessions.retain(|item| item.session_id != session_id);
    insert(key.to_path_buf(), sessions);
}

pub(super) fn remove_all(session_id: &str) {
    let keys: Vec<PathBuf> = entries().iter().map(|entry| entry.key().clone()).collect();
    for key in keys {
        remove(&key, session_id);
    }
}

fn entries() -> &'static DashMap<PathBuf, Sessions> {
    static CACHE: OnceLock<DashMap<PathBuf, Sessions>> = OnceLock::new();
    CACHE.get_or_init(DashMap::new)
}
