//! `codetether connect` — SSH into a designated Ubuntu VM, set up a
//! dedicated forwarded port, run a remote `codetether auth` device-code
//! flow, and pop the verification URL open in the local (Windows) browser.
//!
//! Typical use from a Windows terminal:
//!
//! ```text
//! codetether connect --host vm.lan --user riley
//! ```
//!
//! The command streams the remote authentication output, detects the SSO
//! verification URL, and launches it in the local default browser so the
//! user approves the login on the machine they are physically sitting at.

mod args;
mod browser;
mod execute;
mod preflight;
mod ssh;
#[cfg(test)]
mod ssh_tests;
mod url_scan;

pub use args::ConnectArgs;
pub use execute::execute;
pub use url_scan::scan_url;
