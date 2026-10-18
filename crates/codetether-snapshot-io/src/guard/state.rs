//! Single process-local map of last successful snapshot hashes.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(super) fn store() -> &'static Mutex<HashMap<String, u64>> {
    static STORE: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}
