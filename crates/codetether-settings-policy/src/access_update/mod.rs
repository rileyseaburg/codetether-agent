//! Access-mode transition sequencing and interactive result reporting.
//!
//! Implement [`Host`] for native trust/session state and approval release,
//! then call [`cycle`] to advance and apply the current mode.
mod execute;
mod host;
pub use execute::cycle;
pub use host::Host;
