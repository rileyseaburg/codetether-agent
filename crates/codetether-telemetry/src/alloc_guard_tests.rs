//! Tests for the capacity-guarding allocator.

use super::alloc_guard::CEILING;
use std::ffi::OsString;
use std::sync::atomic::Ordering;

/// `configure` must resolve a non-zero ceiling, floor a too-small override,
/// and leave ordinary allocations (far below the ceiling) untouched.
///
/// Both cases live in one test so they run sequentially: the allocator and
/// its ceiling are process-global, so concurrent `configure` calls would
/// race.
#[test]
fn configure_arms_guard_and_floors_tiny_override() {
    // Deterministic absolute ceiling via env override.
    // SAFETY: single-threaded test setup; no other thread reads the var.
    unsafe { std::env::set_var("CODETETHER_ALLOC_CEILING_MIB", "4096") };
    super::alloc_guard_config::configure(std::env::temp_dir().join("ct-alloc-guard-test"));
    assert_eq!(CEILING.load(Ordering::Relaxed), 4096 * 1024 * 1024);

    // A normal allocation well under the ceiling succeeds verbatim.
    let v: Vec<u8> = Vec::with_capacity(8 * 1024 * 1024);
    assert_eq!(v.capacity(), 8 * 1024 * 1024);

    // A tiny override is floored to the 2 GiB minimum so the guard never
    // trips on modest legitimate allocations.
    // SAFETY: single-threaded test.
    unsafe { std::env::set_var("CODETETHER_ALLOC_CEILING_MIB", "1") };
    super::alloc_guard_config::configure(std::env::temp_dir().join("ct-alloc-guard-test"));
    assert!(CEILING.load(Ordering::Relaxed) >= 2 * 1024 * 1024 * 1024);

    // Restore the inert state so the rest of the suite is unaffected.
    CEILING.store(0, Ordering::Relaxed);
    // SAFETY: single-threaded test teardown.
    unsafe { std::env::remove_var("CODETETHER_ALLOC_CEILING_MIB") };
}

#[test]
fn command_line_tolerates_invalid_unicode() {
    let args = [OsString::from("codetether"), invalid_os_string()];
    let command_line = super::alloc_guard_command::command_line_from(args);
    assert!(command_line.starts_with("codetether "));
    assert!(command_line.contains('\u{fffd}'));
}

#[cfg(unix)]
fn invalid_os_string() -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(vec![0xff])
}

#[cfg(not(unix))]
fn invalid_os_string() -> OsString {
    OsString::from("�")
}
