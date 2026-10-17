//! Explicit release of unused glibc heap pages.

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) fn trim() -> bool {
    // SAFETY: a zero pad requests release of every wholly unused heap page.
    unsafe { libc::malloc_trim(0) != 0 }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
pub(super) fn trim() -> bool {
    false
}
