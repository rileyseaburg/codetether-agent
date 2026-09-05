//! Scoped opt-in environment isolation for the Codex safety integration test.

use std::ffi::OsString;
use std::sync::OnceLock;
use tokio::sync::Mutex;

pub(super) const OPT_IN_ENV: &str = "CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND";

pub(super) fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub(super) struct EnvGuard {
    old_value: Option<OsString>,
}

impl EnvGuard {
    pub(super) fn without_opt_in() -> Self {
        let old_value = std::env::var_os(OPT_IN_ENV);
        unsafe { std::env::remove_var(OPT_IN_ENV) };
        Self { old_value }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old_value {
            Some(value) => unsafe { std::env::set_var(OPT_IN_ENV, value) },
            None => unsafe { std::env::remove_var(OPT_IN_ENV) },
        }
    }
}
