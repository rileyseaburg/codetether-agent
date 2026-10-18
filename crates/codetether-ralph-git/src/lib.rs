//! Policy-gated Git operations for Ralph workspace and story commits.
//!
//! Implement [Host] with the native invocation-policy evaluator. [checkout]
//! performs the tool's guarded checkout; [checkout_or_create] preserves the
//! run-start branch fallback. Story commits attach Ralph run provenance and
//! retain the original distinction between logged and ignored Git failures.
mod git_checkout;
mod git_commit;
mod git_commit_dir;
mod host;
mod policy;
mod provenance;
use codetether_provenance::{
    ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance_blocking,
};
use codetether_ralph_core::UserStory;
pub use git_checkout::git_checkout as checkout_or_create;
pub use git_commit::commit_story;
pub use git_commit_dir::commit_in_dir;
pub use host::Host;
pub use policy::checkout;
use provenance::commit_provenance;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tracing::{info, warn};
