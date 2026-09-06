// Platform-specific implementation and portable input-routing rules.
pub(crate) mod platform;
mod execution;
pub mod worker;
#[cfg(any(target_os = "windows", test))]
pub(crate) mod capture_limits;
#[cfg(any(target_os = "windows", test))]
pub(crate) mod capture_preview;
#[cfg(any(target_os = "windows", test))]
mod routing;
#[cfg(any(target_os = "windows", test))]
pub(crate) mod shadow;