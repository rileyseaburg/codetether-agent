//! Compatibility paths for shared A2A stream protocols and transport.
//!
//! Stream implementation belongs to `codetether-a2a-stream`. Socket defaults
//! and TCP observations retain their provider-core and telemetry owners.
pub use codetether_a2a_stream::{
    backoff, breaker, classify, cursor, event_id, frame, lifecycle, probe, quic, resume_request,
    resync, staging,
};
pub use codetether_provider_core::socket_opts;
pub use codetether_telemetry::tcp_info;
