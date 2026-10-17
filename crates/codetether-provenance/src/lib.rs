//! Execution provenance, runtime identity, and Git attribution.
//!
//! Build an [`ExecutionProvenance`] record, apply claim or run identifiers, and
//! use the commit helpers to attach trailers and stable author information.

mod claim;
mod forgejo_identity;
mod git;
mod hook;
mod identity;
mod runtime_identity;
mod signature;
mod trailers;
mod types;

pub use claim::ClaimProvenance;
pub use forgejo_identity::forgejo_agent_identity;
pub use git::commit_args_with_signature_policy;
pub use git::enrich_from_repo;
pub use git::{git_commit_with_provenance, git_commit_with_provenance_blocking};
pub use hook::commit_msg_hook_script;
pub use hook::ensure_provenance_trailers;
pub use hook::install_commit_msg_hook;
pub use hook::{commit_editmsg_path, repo_root};
pub use identity::git_identity_env_vars;
pub use identity::{git_author_email, git_author_name};
pub use runtime_identity::RuntimePrincipal;
pub use runtime_identity::bind_runtime_agent_identity;
pub use runtime_identity::{ensure_runtime_agent_identity, runtime_agent_identity};
pub use runtime_identity::{runtime_persona_id, runtime_spiffe_id};
pub use signature::sign_provenance;
pub use trailers::provenance_trailers;
pub use types::{ExecutionOrigin, ExecutionProvenance};

#[cfg(test)]
#[path = "types_tests.rs"]
mod types_tests;

#[cfg(test)]
mod consolidation;

/// Body-bound worker request proofs.
pub mod worker_proof;
