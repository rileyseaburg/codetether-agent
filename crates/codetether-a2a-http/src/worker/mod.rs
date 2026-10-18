//! Worker HTTP transport helpers for registration, claims and releases.
//!
//! Callers build and sign requests. This module constructs the streaming client,
//! handles response status/body decoding, and defines the release payload.
mod client;
mod registration_response;
mod release;
mod release_payload;
mod summary;
pub use client::build_worker_http_client;
pub use registration_response::require_success;
pub use release::send_release;
pub use release_payload::Payload;
pub use summary::summarize_response_body;

mod pending;
pub use pending::pending_tasks;

mod heartbeat_payload;
mod heartbeat_response;
mod stream_error;
pub use heartbeat_payload::prepare_extended_heartbeat;
pub use heartbeat_response::check_extended_heartbeat;
pub use stream_error::connect_error;

/// Signed extended-heartbeat delivery.
pub mod extended_heartbeat;
/// Signed task output delivery.
pub mod output;
/// Canonical worker request signatures and proof headers.
pub mod proof;

mod claim_request;
pub use claim_request::build_claim_request;
mod registration;
pub use registration::register_worker_payload;
mod register;
pub use register::register_worker;
mod release_request;
pub use release_request::build_release_request;

/// Cognition metadata retrieval for worker heartbeats.
pub mod cognition;
mod heartbeat_post;
pub use heartbeat_post::post_heartbeat;

/// Worker state and progress snapshot assembly for heartbeats.
pub mod heartbeat_snapshot;

mod heartbeat_loop;
pub use heartbeat_loop::start_heartbeat;

/// Claim response handling and progress transitions.
pub mod claim;

/// Task outcome release and active-progress cleanup.
pub mod completion;

/// Bus-aware callbacks feeding signed worker output requests.
pub mod output_callback;
