//! In-process background RLM result cache.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::disk;
pub(super) use super::key::key;

enum Entry {
    Running,
    Ready(String),
}

static CACHE: OnceLock<Mutex<HashMap<u64, Entry>>> = OnceLock::new();

pub(super) fn ready(key: u64) -> Option<String> {
    let mut map = CACHE.get_or_init(default).lock().ok()?;
    if let Some(Entry::Ready(value)) = map.get(&key) {
        return Some(value.clone());
    }
    if map.contains_key(&key) {
        return None;
    }
    let value = disk::read(key)?;
    map.insert(key, Entry::Ready(value.clone()));
    Some(value)
}

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

fn default() -> Mutex<HashMap<u64, Entry>> {
    Mutex::new(HashMap::new())
}
