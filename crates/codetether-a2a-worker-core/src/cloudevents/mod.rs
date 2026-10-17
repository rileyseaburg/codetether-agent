//! Structured and binary CloudEvent ingestion for task receivers.
//!
//! [CloudEvent] is the normalized record. [parse_cloud_event] accepts an owned
//! JSON body plus a host-provided header lookup, without depending on HTTP frameworks.
mod event;
mod parse;
pub use event::CloudEvent;
pub use parse::parse_cloud_event;
