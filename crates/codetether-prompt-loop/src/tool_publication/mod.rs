//! Ordered publication of completed and synthetic tool results.
//!
//! [complete] publishes bus/archive/UI output before inserting the routed tool
//! result and persisting it. [record] inserts synthetic results without those
//! completion-only effects. Native state and services are supplied by [Host].
mod complete;
mod host;
mod record;
pub use complete::complete;
pub use host::Host;
pub use record::record;
