//! Shared process-environment isolation for approval and policy tests.

use crate::config::{AccessMode, Config};
use std::path::Path;

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) struct ScopedEnv;

impl ScopedEnv {
    pub(crate) fn access(mode: AccessMode) -> Self {
        Config::apply_process_access_mode_override(Some(mode));
        Self
    }

    pub(crate) fn data_dir_with_access(path: &Path, mode: AccessMode) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self::access(mode)
    }
}

impl Drop for ScopedEnv {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        Config::apply_process_access_mode_override(None);
    }
}

pub(crate) fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    let guard = ENV_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    crate::approval::session_grants::reset();
    crate::approval::session_command_grants::reset();
    Config::apply_process_access_mode_override(None);
    guard
}
