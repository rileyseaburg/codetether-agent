//! Serialize editor tests and restore their process-wide settings on exit.

use std::ffi::OsString;
use tokio::sync::{Mutex, MutexGuard};

static LOCK: Mutex<()> = Mutex::const_new(());
const KEYS: [&str; 2] = ["CODETETHER_VSCODE_BIN", "CODETETHER_WORKTREE_AUTO_VSCODE"];

pub(crate) struct EditorEnvironment {
    _lock: MutexGuard<'static, ()>,
    values: [Option<OsString>; 2],
    tui_active: bool,
}

impl EditorEnvironment {
    pub(crate) async fn lock() -> Self {
        let lock = LOCK.lock().await;
        Self {
            _lock: lock,
            values: KEYS.map(std::env::var_os),
            tui_active: crate::is_tui_active(),
        }
    }
}

impl Drop for EditorEnvironment {
    fn drop(&mut self) {
        for (key, value) in KEYS.into_iter().zip(&self.values) {
            // SAFETY: editor tests hold LOCK throughout mutation and restoration.
            unsafe {
                match value {
                    Some(value) => std::env::set_var(key, value),
                    None => std::env::remove_var(key),
                }
            }
        }
        crate::set_tui_active(self.tui_active);
    }
}
