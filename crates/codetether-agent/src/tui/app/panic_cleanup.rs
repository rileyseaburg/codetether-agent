//! Shared panic-hook lifetime and recoverable-render scope.
pub(super) use codetether_terminal_runtime::{
    PanicHookGuard, begin_render_recovery, install_panic_cleanup_hook,
};
