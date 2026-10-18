//! Process-environment isolation for native TUI session-loader tests.

use std::{ffi::OsString, path::Path};

static LOCK: parking_lot::Mutex<()> = parking_lot::Mutex::new(());

pub(super) struct Guard {
    data_dir: Option<OsString>,
    resume_window: Option<OsString>,
    _lock: parking_lot::MutexGuard<'static, ()>,
}

impl Guard {
    pub(super) fn set(data_dir: &Path) -> Self {
        let lock = LOCK.lock();
        let guard = Self {
            data_dir: std::env::var_os("CODETETHER_DATA_DIR"),
            resume_window: std::env::var_os("CODETETHER_SESSION_RESUME_WINDOW"),
            _lock: lock,
        };
        unsafe {
            std::env::set_var("CODETETHER_DATA_DIR", data_dir);
            std::env::set_var("CODETETHER_SESSION_RESUME_WINDOW", "2");
        }
        guard
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        restore("CODETETHER_DATA_DIR", self.data_dir.take());
        restore(
            "CODETETHER_SESSION_RESUME_WINDOW",
            self.resume_window.take(),
        );
    }
}

fn restore(name: &str, value: Option<OsString>) {
    unsafe {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}
