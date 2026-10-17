//! HTTP possession-proof headers for canonical worker identities.
//!
//! [apply] uses an explicit principal; [apply_configured] reads the shared
//! runtime principal. Both preserve requests for workers that do not require
//! canonical Forgejo proof. JSON resource binding is exported as [mutation_resource].
mod configured;
mod identity;
pub use codetether_provenance::worker_proof::for_json as mutation_resource;
pub use configured::apply_configured;
pub use identity::apply;
