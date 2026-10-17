use super::{AccessMode, Config};
use std::sync::{OnceLock, RwLock};

static PROCESS_ACCESS_MODE: OnceLock<RwLock<Option<AccessMode>>> = OnceLock::new();

impl Config {
    pub fn apply_process_access_mode_override(access_mode: Option<AccessMode>) {
        *slot().write().unwrap_or_else(|e| e.into_inner()) = access_mode;
    }
}

pub fn process_access_mode_override() -> Option<AccessMode> {
    *slot().read().unwrap_or_else(|e| e.into_inner())
}

fn slot() -> &'static RwLock<Option<AccessMode>> {
    PROCESS_ACCESS_MODE.get_or_init(|| RwLock::new(None))
}
