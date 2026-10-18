//! Serialized environment setup used by the configuration regression.
use crate::ThreadStore;
use std::sync::Mutex;
static ENV_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn store_from_config(root: &std::path::Path) -> ThreadStore {
    let _guard = ENV_LOCK.lock().unwrap();
    // SAFETY: this focused test serializes access with ENV_LOCK and restores
    // the variable before returning.
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", root) };
    let store = ThreadStore::from_config().unwrap();
    // SAFETY: paired with the guarded set_var above.
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    store
}
