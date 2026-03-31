mod claim;
mod git;
mod hook;
mod identity;
mod repo;
mod signature;
mod types;

pub use claim::ClaimProvenance;
pub use git::{git_commit_with_provenance, git_commit_with_provenance_blocking};
pub use hook::install_commit_msg_hook;
pub use identity::{git_author_email, git_author_name};
pub use repo::enrich_from_repo;
pub use signature::sign_provenance;
pub use types::{ExecutionOrigin, ExecutionProvenance};
