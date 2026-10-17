//! Unified-diff parsing, grouping and in-memory application.
//!
//! Parse patches with [parse_patch], group [PatchHunk] values by target, then
//! use [apply_hunk] against existing text. The caller owns filesystem access,
//! path validation and approvals. [approval_resource_from_patch] reproduces
//! the tool's stable target-resource identity.
mod group;
mod hunk_apply;
mod hunk_builder;
mod parser;
mod resource;
mod types;
pub use group::{by_file, files};
pub use hunk_apply::apply as apply_hunk;
pub use parser::parse_patch;
pub use resource::resource as approval_resource;
pub use types::PatchHunk;

/// Return the approval resource string for a unified diff patch.
///
/// # Arguments
/// * `patch` — Unified-diff text to parse.
/// # Returns
/// Identity derived from sorted, unique parsed target paths.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_core::patch::approval_resource_from_patch(""), "workspace");
/// ```
pub fn approval_resource_from_patch(patch: &str) -> String {
    let hunks = parser::parse_patch(patch);
    resource::resource(&group::files(&hunks))
}
