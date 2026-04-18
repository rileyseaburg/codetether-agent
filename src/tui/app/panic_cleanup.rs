use std::{panic, sync::Arc};

use super::terminal_state::restore_terminal_state;

type PanicHook = dyn Fn(&panic::PanicHookInfo<'_>) + Send + Sync + 'static;

pub(super) struct PanicHookGuard {
    previous: Arc<PanicHook>,
}

pub(super) fn install_panic_cleanup_hook() -> PanicHookGuard {
    let previous: Arc<PanicHook> = panic::take_hook().into();
    let hook = Arc::clone(&previous);
    panic::set_hook(Box::new(move |info| {
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