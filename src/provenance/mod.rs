mod claim;
mod claim_apply;
mod commit_args;
mod forgejo_identity;
mod git;
mod git_identity;
mod hook;
mod hook_chmod;
mod hook_path;
mod hook_script;
mod hook_seed;
mod identity;
mod repo;
mod runtime_identity;
mod runtime_identity_read;
mod runtime_identity_store;
mod runtime_identity_write;
mod runtime_principal;
mod signature;
mod trailer_forgejo;
mod trailer_identity;
mod trailer_optional;
mod trailer_run;
mod trailers;
mod types;

pub use claim::ClaimProvenance;
pub use commit_args::commit_args_with_signature_policy;
pub use forgejo_identity::forgejo_agent_identity;
pub use git::{git_commit_with_provenance, git_commit_with_provenance_blocking};
pub use git_identity::git_identity_env_vars;
pub use hook::install_commit_msg_hook;
pub use hook_path::{commit_editmsg_path, repo_root};
pub use hook_script::commit_msg_hook_script;
pub use hook_seed::ensure_provenance_trailers;
pub use identity::{git_author_email, git_author_name};
pub use repo::enrich_from_repo;
pub(crate) use runtime_identity::bind_runtime_agent_identity;
pub use runtime_identity::{ensure_runtime_agent_identity, runtime_agent_identity};
pub use runtime_principal::RuntimePrincipal;
pub(crate) use runtime_principal::{runtime_persona_id, runtime_spiffe_id};
pub use signature::sign_provenance;
pub use trailers::provenance_trailers;
pub use types::{ExecutionOrigin, ExecutionProvenance};

#[cfg(test)]
#[path = "types_tests.rs"]
mod types_tests;
