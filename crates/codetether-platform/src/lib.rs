//! Native operating-system services shared by CodeTether frontends.
//!
//! [capture] owns platform-independent pixel bounds and image encoding.
//! Windows integrations provide browser discovery, window/input operations,
//! and voice adapters. [unix] retains the non-Windows capability stub.
pub mod capture;
#[cfg(not(target_os = "windows"))]
pub mod unix;
#[cfg(target_os = "windows")]
pub mod windows;
