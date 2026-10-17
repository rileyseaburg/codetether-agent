//! Landlock rule planning, pre-exec enforcement and platform fallback.
//!
//! [prepare] derives filesystem rules from a sandbox policy. [apply] installs
//! them for a future child; [kernel_available] reports ABI availability.
#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod imp;

#[cfg(not(target_os = "linux"))]
#[path = "unsupported.rs"]
mod imp;

pub use imp::{Rules, apply, prepare};

/// Returns `true` when the running kernel can enforce Landlock confinement.
///
/// Used by sandbox availability checks so that a host where bwrap fails but
/// Landlock works is still treated as sandbox-capable for direct execution.
pub fn kernel_available() -> bool {
    imp::abi_version() > 0
}
