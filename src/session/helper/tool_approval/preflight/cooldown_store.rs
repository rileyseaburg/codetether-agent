//! Shared server state, with expiry as a fallback for abandoned background work.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Mutex, MutexGuard, OnceLock},
};
use tokio::time::Instant;

type Entries = HashMap<(PathBuf, String), (Instant, String)>;
static ENTRIES: OnceLock<Mutex<Entries>> = OnceLock::new();

pub(super) fn entries() -> MutexGuard<'static, Entries> {
    let mut entries = ENTRIES
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    entries.retain(|_, (until, _)| *until > Instant::now());
    entries
}
