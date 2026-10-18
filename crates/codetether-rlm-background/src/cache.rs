//! In-process background RLM result cache.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(super) use super::cache_read::ready;
use super::disk;
pub(super) use super::key::key;

pub(super) enum Entry {
    Running,
    Ready(String),
}

pub(super) static CACHE: OnceLock<Mutex<HashMap<u64, Entry>>> = OnceLock::new();

pub(super) fn claim(key: u64) -> bool {
    let Ok(mut map) = CACHE.get_or_init(default).lock() else {
        return false;
    };
    if map.contains_key(&key) {
        return false;
    }
    map.insert(key, Entry::Running);
    true
}

pub(super) fn complete(key: u64, summary: String) {
    if let Ok(mut map) = CACHE.get_or_init(default).lock() {
        disk::write(key, &summary);
        map.insert(key, Entry::Ready(summary));
    }
}

pub(super) fn fail(key: u64) {
    if let Ok(mut map) = CACHE.get_or_init(default).lock() {
        map.remove(&key);
    }
}

pub(super) fn default() -> Mutex<HashMap<u64, Entry>> {
    Mutex::new(HashMap::new())
}
