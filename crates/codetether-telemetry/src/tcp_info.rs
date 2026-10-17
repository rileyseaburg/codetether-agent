//! Kernel TCP observations for transport telemetry.
//!
//! Use `sample` to read a live socket, or `TcpInfoSnapshot` to pass an observation.

mod snapshot;
pub use snapshot::TcpInfoSnapshot;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::sample;
#[cfg(not(target_os = "linux"))]
mod fallback;
#[cfg(not(target_os = "linux"))]
pub use fallback::sample;
#[cfg(all(test, target_os = "linux"))]
mod tests;
