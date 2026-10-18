//! Serialized Morph configuration with unwind-safe environment restoration.

use std::{ffi::OsString, sync::OnceLock};
use tokio::sync::Mutex;

pub(super) fn lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub(super) struct EnvGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvGuard {
    pub(super) fn new(key: &'static str) -> Self {
        Self {
            key,
            previous: std::env::var_os(key),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.previous {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

pub(super) fn enable(base_url: &str) -> [EnvGuard; 4] {
    [
        ("CODETETHER_MORPH_TOOL_BACKEND", "1"),
        ("OPENROUTER_API_KEY", "test-key"),
        ("CODETETHER_OPENROUTER_BASE_URL", base_url),
        ("CODETETHER_MORPH_TOOL_MODEL", "morph/morph-v3-large"),
    ]
    .map(|(key, value)| {
        let guard = EnvGuard::new(key);
        unsafe { std::env::set_var(key, value) };
        guard
    })
}
