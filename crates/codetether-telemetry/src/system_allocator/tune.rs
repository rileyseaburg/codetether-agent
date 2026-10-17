//! Early glibc arena tuning.

use std::sync::Once;

use super::config::Settings;

pub(super) fn initialize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        apply(Settings::default());
    });
}

pub(super) fn configure(settings: Settings) {
    initialize();
    let applied = apply(settings);
    tracing::info!(
        arena_max = settings.arena_max,
        trim_kib = settings.trim_bytes / 1024,
        applied,
        "Configured system allocator"
    );
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn apply(settings: Settings) -> bool {
    // SAFETY: mallopt accepts integer parameters and mutates glibc allocator policy.
    unsafe {
        libc::mallopt(libc::M_ARENA_MAX, settings.arena_max) != 0
            && libc::mallopt(libc::M_TRIM_THRESHOLD, settings.trim_bytes) != 0
    }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn apply(_settings: Settings) -> bool {
    false
}
