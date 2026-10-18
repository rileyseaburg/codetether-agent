//! Native session specialization of shared durable event transport.
//!
//! The shared crate owns typed durable writes, optional legacy forwarding,
//! and broadcast delivery. Native sessions select the event snapshot type.
pub use codetether_session_bus::{DurableSink, NoopSink};
/// Event bus carrying application session snapshots.
pub type SessionBus = codetether_session_bus::DurableSessionBus<super::Session>;
