//! Runtime workspace commit policy and bindings for shared story merging.
//!
//! Application adapters retain live resources and call shared Ralph services.
//! Methods preserve their original visibility within the Ralph loop owner.
use super::*;
mod git_checkout;
mod git_commit;
mod git_commit_dir;
mod host;
mod merge_story;
