// Platform-specific implementation and portable input-routing rules.
#[cfg(any(target_os = "windows", target_os = "linux", test))]
pub(crate) mod capture_limits;
#[cfg(any(target_os = "windows", target_os = "linux", test))]
pub(crate) mod capture_preview;
mod execution;
pub(crate) mod platform;
#[cfg(any(target_os = "windows", test))]
mod routing;
#[cfg(any(target_os = "windows", test))]
pub(crate) mod shadow;
pub mod worker;
