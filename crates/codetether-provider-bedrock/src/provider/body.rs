//! Bedrock Converse request-body compatibility exports.
//!
//! Request construction and model controls are owned by the protocol crate.
pub(super) use codetether_bedrock_protocol::audit;
pub use codetether_bedrock_protocol::body::build_converse_body;
pub(super) use codetether_bedrock_protocol::body::{fields, inference};
