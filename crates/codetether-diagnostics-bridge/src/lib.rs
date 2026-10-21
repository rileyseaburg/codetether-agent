//! Private diagnostic snapshots shared by native LSP producers and editors.
//!
//! [`Publisher`] owns one atomic JSON snapshot and removes it on drop.
//! Consumers scan the directory, filter by canonical workspace URI, and check
//! both PID liveness and heartbeat age. No session or conversation data is stored.
//! ```no_run
//! use codetether_diagnostics_bridge::{Publisher, Diagnostics};
//! let publisher = Publisher::new("file:///workspace", "rust-analyzer")?;
//! publisher.publish(&Diagnostics::new())?;
//! # Ok::<(), anyhow::Error>(())
//! ```
mod directory;
#[cfg(test)]
mod privacy_tests;
mod publish;
mod publisher;
mod snapshot;
mod uri;
pub use publisher::Publisher;
pub use snapshot::{Diagnostics, Snapshot};
#[cfg(test)]
mod tests;
