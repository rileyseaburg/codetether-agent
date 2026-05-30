//! Startup configuration for the
//! [capacity-guarding allocator](super::alloc_guard).
//!
//! Resolves the per-allocation ceiling from physical RAM (default 75%) with
//! env overrides, then records the crash-report spool directory so the trip
//! handler can write a breadcrumb.

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use super::alloc_guard::{CEILING, SPOOL_DIR};

/// Absolute ceiling override, in MiB. Wins over the RAM-percent path.
const ENV_CEILING_MIB: &str = "CODETETHER_ALLOC_CEILING_MIB";
/// Ceiling as a percent of physical RAM (default 75, clamped to 10..=95).
const ENV_CEILING_RAM_PCT: &str = "CODETETHER_ALLOC_CEILING_RAM_PCT";

/// Floor so the guard never trips on modest legitimate allocations, even on
/// small machines or when RAM detection fails.
const MIN_CEILING_BYTES: usize = 2 * 1024 * 1024 * 1024;
/// Assumed RAM when detection is unavailable (e.g. non-Unix) and no
/// override is set — keeps the guard armed at a sane level everywhere.
const FALLBACK_RAM_BYTES: usize = 16 * 1024 * 1024 * 1024;

/// Arm the allocator guard. Call once, early in process startup. Idempotent
/// for the spool dir; the ceiling is simply restored on repeat calls.
pub fn configure(spool_dir: PathBuf) {
    let _ = SPOOL_DIR.set(spool_dir);
    CEILING.store(resolve_ceiling(), Ordering::Relaxed);
}

fn resolve_ceiling() -> usize {
    if let Some(mib) = env_usize(ENV_CEILING_MIB) {
        return mib.saturating_mul(1024 * 1024).max(MIN_CEILING_BYTES);
    }
    let pct = env_usize(ENV_CEILING_RAM_PCT).unwrap_or(75).clamp(10, 95);
    let ram = match total_ram_bytes() {
        0 => FALLBACK_RAM_BYTES,
        n => n,
    };
    (ram / 100).saturating_mul(pct).max(MIN_CEILING_BYTES)
}

fn env_usize(var: &str) -> Option<usize> {
    std::env::var(var).ok()?.trim().parse().ok()
}

#[cfg(unix)]
fn total_ram_bytes() -> usize {
    // SAFETY: `sysconf` takes a constant name and is always valid to call.
    let pages = unsafe { libc::sysconf(libc::_SC_PHYS_PAGES) };
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if pages > 0 && page > 0 {
        (pages as usize).saturating_mul(page as usize)
    } else {
        0
    }
}

#[cfg(not(unix))]
fn total_ram_bytes() -> usize {
    0
}
