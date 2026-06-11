#[cfg(target_os = "linux")]
#[path = "sandbox_landlock_linux.rs"]
mod imp;

#[cfg(not(target_os = "linux"))]
#[path = "sandbox_landlock_unsupported.rs"]
mod imp;

pub(super) use imp::{Rules, apply, prepare};
