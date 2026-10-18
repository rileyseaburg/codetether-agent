//! Loaded-transcript synchronization and retained message selection.
//!
//! Implement [View] to expose native history and presentation operations, then
//! call [sync_messages_from_source]. [Metrics] resets completed-turn samples.
mod host;
mod metrics;
mod service;
pub mod session;
mod window;
pub use host::View;
pub use metrics::Metrics;
pub use service::sync_messages_from_source;
pub use window::recent;
