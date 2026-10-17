//! A2A protobuf contracts and gRPC wire conversions.
//!
//! [proto] exposes generated tonic/prost messages and services. [bridge]
//! converts between those wire types and `codetether-a2a-core` contracts.
//!
//! ```rust
//! use codetether_a2a_grpc::{bridge, proto};
//! let message = proto::Message::default();
//! let local = bridge::proto_message_to_local(&message);
//! assert_eq!(local.message_id, message.message_id);
//! ```
pub mod bridge;
pub mod proto;

/// Task storage, update subscriptions and tonic service implementation.
pub mod server;
