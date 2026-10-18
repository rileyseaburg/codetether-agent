//! Fetch ready cached summaries or hydrate them from disk.
use super::cache::{CACHE, Entry, default};
use super::disk;

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
