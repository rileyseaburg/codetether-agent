//! Seccomp filter generation and inherited profile descriptor ownership.
//!
//! [prepare] builds a filter profile on supported Linux x86_64 hosts.
//! Retain the returned [Program] while its descriptor is passed to a launcher.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[path = "linux.rs"]
mod imp;

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
#[path = "unsupported.rs"]
mod imp;

pub use imp::{Program, prepare};
