//! Apply unified diff patches to workspace files.

#[path = "patch/apply.rs"]
mod apply;
#[path = "patch/approval.rs"]
mod approval;
#[path = "patch/approval_flow.rs"]
mod approval_flow;
#[path = "patch/args.rs"]
mod args;
#[path = "patch/file_io.rs"]
mod file_io;
#[path = "patch/group.rs"]
mod group;
#[path = "patch/hunk_apply.rs"]
mod hunk_apply;
#[path = "patch/hunk_builder.rs"]
mod hunk_builder;
#[path = "patch/metadata.rs"]
mod metadata;
#[path = "patch/parser.rs"]
mod parser;
#[path = "patch/path_guard.rs"]
mod path_guard;
#[path = "patch/pipeline.rs"]
mod pipeline;
#[path = "patch/result.rs"]
mod result;
#[path = "patch/schema.rs"]
mod schema;
#[path = "patch/success.rs"]
mod success;
#[path = "patch/tool.rs"]
mod tool;
#[path = "patch/types.rs"]
mod types;

pub use self::tool::ApplyPatchTool;

/// Return the approval resource string for a unified diff patch.
pub fn approval_resource_from_patch(patch: &str) -> String {
    let hunks = parser::parse_patch(patch);
    approval::resource(&group::files(&hunks))
}

#[cfg(test)]
#[path = "patch/approval_tests.rs"]
mod approval_tests;
#[cfg(test)]
#[path = "patch/approval_verify_tests.rs"]
mod approval_verify_tests;
#[cfg(test)]
#[path = "patch/preview_tests.rs"]
mod preview_tests;
#[cfg(test)]
#[path = "patch/test_support.rs"]
mod test_support;
