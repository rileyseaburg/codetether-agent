//! Canonical worker proof, registration, and claim request boundary.

use super::WorkerTaskRuntime;

use codetether_a2a_http::worker as worker_registration_response;

pub(super) use codetether_a2a_http::worker::proof::apply_configured as apply_configured_proof;
pub use codetether_native_worker::register_worker;
pub(super) use codetether_provenance::worker_proof::for_json as mutation_resource;
