//! Native network mux composition and application-facing control.
//!
//! [execute] runs CLI lifecycle commands. [control] exposes session and window
//! operations, while [coordination] keeps the shared workspace lease state.
//! Session lookups and clipboard reads use their owning service crates.
mod mux;
#[cfg(test)]
use codetether_native_approval as approval;
pub use mux::*;
