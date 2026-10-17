//! Workspace commits, merge handling and conflict resolution.
//!
//! Application adapters retain live resources and call shared Ralph services.
//! Methods preserve their original visibility within the Ralph loop owner.
use super::*;
mod conflict_complete;
mod conflict_execute;
mod conflict_invoke;
mod conflict_merge;
mod conflict_result;
mod conflict_verify;
mod conflicts;
mod git_checkout;
mod git_commit;
mod git_commit_dir;
mod merge_dispatch;
mod merge_nonconflict;
mod merge_story;
mod merge_success;
mod merge_verify;
mod merge_without_worktree;
mod provenance;
