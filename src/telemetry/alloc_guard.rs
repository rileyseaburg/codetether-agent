//! Capacity-guarding global allocator.
//!
//! Wraps the system allocator and trips on any *single* allocation request
//! larger than a configured ceiling — far above any legitimate working set
//! for this agent. A corrupted capacity computation that tries to reserve
//! tens of gigabytes (the `memory allocation of N bytes failed` class of
//! crash) is caught here: the [`report`](super::alloc_guard_report) module
//! captures a backtrace, spools a crash report, and aborts cleanly instead
//! of leaving the OS OOM-killer to `SIGKILL` the process with no clue why.
//!
//! The hot path is a single relaxed atomic load plus a compare. The ceiling
//! is `0` (disabled) until
//! [`configure`](super::alloc_guard_config::configure) runs at startup, so
//! allocations made before configuration always pass straight through.

use std::alloc::{GlobalAlloc, Layout, System};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Per-allocation hard ceiling in bytes. `0` means "not yet configured".
pub(crate) static CEILING: AtomicUsize = AtomicUsize::new(0);

/// Crash-report spool directory, recorded at configuration time so the trip
/// handler can write without re-deriving platform paths under duress.
pub(crate) static SPOOL_DIR: OnceLock<PathBuf> = OnceLock::new();

/// The installed global allocator. Delegates every request to [`System`],
/// guarding only the requested size.
pub struct GuardAlloc;

#[inline]
fn check(size: usize) {
    let ceiling = CEILING.load(Ordering::Relaxed);
    if ceiling != 0 && size > ceiling {
        super::alloc_guard_report::trip(size, ceiling);
    }
}

unsafe impl GlobalAlloc for GuardAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        check(layout.size());
        unsafe { System.alloc(layout) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        check(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        check(new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
