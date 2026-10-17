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
