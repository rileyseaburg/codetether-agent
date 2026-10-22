//! Bounded registry; unrelated journal reads do not share a disk-I/O lock.
use super::entry::Entry;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
    time::Instant,
};

type Cell = Arc<Mutex<Option<Entry>>>;
struct Slot {
    used: Instant,
    cell: Cell,
}
static CACHE: LazyLock<Mutex<HashMap<PathBuf, Slot>>> = LazyLock::new(Mutex::default);

pub(super) fn get(path: &Path) -> Cell {
    let mut cache = CACHE.lock();
    if let Some(slot) = cache.get_mut(path) {
        slot.used = Instant::now();
        return Arc::clone(&slot.cell);
    }
    if cache.len() >= 64
        && let Some(oldest) = cache
            .iter()
            .min_by_key(|(_, slot)| slot.used)
            .map(|(path, _)| path.clone())
    {
        cache.remove(&oldest);
    }
    let cell = Arc::new(Mutex::new(None));
    cache.insert(
        path.to_path_buf(),
        Slot {
            used: Instant::now(),
            cell: Arc::clone(&cell),
        },
    );
    cell
}
