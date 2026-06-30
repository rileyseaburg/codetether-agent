#[cfg(target_os = "linux")]
#[path = "sandbox_landlock_linux.rs"]
mod imp;

#[cfg(not(target_os = "linux"))]
#[path = "sandbox_landlock_unsupported.rs"]
mod imp;

pub(super) use imp::{Rules, apply, prepare};

/// Returns `true` when the running kernel can enforce Landlock confinement.
///
/// Used by sandbox availability checks so that a host where bwrap fails but
/// Landlock works is still treated as sandbox-capable for direct execution.
pub(super) fn kernel_available() -> bool {
    imp::abi_version() > 0
}
