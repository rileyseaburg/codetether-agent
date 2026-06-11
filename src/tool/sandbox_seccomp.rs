#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[path = "sandbox_seccomp_linux.rs"]
mod imp;

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
#[path = "sandbox_seccomp_unsupported.rs"]
mod imp;

pub(super) use imp::{Program, prepare};
