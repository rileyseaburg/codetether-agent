//! Panic-hook installation and restoration around interactive terminal ownership.
use std::{panic, sync::Arc};

use crate::{recovery_scope, restore_terminal_state};

type PanicHook = dyn Fn(&panic::PanicHookInfo<'_>) + Send + Sync + 'static;

/// Restores the previous panic hook when dropped.
///
/// # Examples
/// ```no_run
/// let _hook = codetether_terminal_runtime::install_panic_cleanup_hook();
/// ```
pub struct PanicHookGuard {
    previous: Arc<PanicHook>,
}

/// Install terminal cleanup before forwarding panics to the previous hook.
///
/// # Returns
/// A guard that restores the prior hook at scope exit.
///
/// # Panics
/// Panics if the standard panic-hook API is called from a panicking thread.
///
/// # Examples
/// ```no_run
/// let _hook = codetether_terminal_runtime::install_panic_cleanup_hook();
/// ```
pub fn install_panic_cleanup_hook() -> PanicHookGuard {
    let previous: Arc<PanicHook> = panic::take_hook().into();
    let hook = Arc::clone(&previous);
    panic::set_hook(Box::new(move |info| {
        if recovery_scope::active() {
            return;
        }
        restore_terminal_state();
        hook(info);
    }));
    PanicHookGuard { previous }
}

impl Drop for PanicHookGuard {
    fn drop(&mut self) {
        let previous = Arc::clone(&self.previous);
        panic::set_hook(Box::new(move |info| previous(info)));
    }
}
