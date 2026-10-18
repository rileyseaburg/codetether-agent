//! Canonical worker proof, registration, and claim request boundary.

use super::WorkerTaskRuntime;

#[path = "worker_configured_proof.rs"]
mod worker_configured_proof;
#[path = "worker_mutation_resource.rs"]
mod worker_mutation_resource;
use codetether_a2a_http::worker as worker_registration_response;

pub use codetether_native_worker::register_worker;
pub(super) use worker_configured_proof::apply as apply_configured_proof;
pub(super) use worker_mutation_resource::for_json as mutation_resource;
