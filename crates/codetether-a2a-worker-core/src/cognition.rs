//! Cognition heartbeat settings and wire snapshots.

mod config;
mod snapshots;
pub use config::CognitionHeartbeatConfig;
pub use snapshots::{CognitionLatestSnapshot, CognitionStatusSnapshot};
