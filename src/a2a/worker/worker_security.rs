//! Canonical worker proof, registration, and claim request boundary.

use super::WorkerTaskRuntime;

#[path = "worker_claim_request.rs"]
mod worker_claim_request;
#[path = "worker_configured_proof.rs"]
mod worker_configured_proof;
#[path = "worker_identity_proof.rs"]
mod worker_identity_proof;
#[path = "worker_mutation_resource.rs"]
mod worker_mutation_resource;
#[path = "worker_registration.rs"]
mod worker_registration;
#[path = "worker_registration_response.rs"]
mod worker_registration_response;

pub(super) use worker_claim_request::build as build_claim_request;
pub(super) use worker_configured_proof::apply as apply_configured_proof;
pub(super) use worker_mutation_resource::for_json as mutation_resource;
pub use worker_registration::register_worker;
