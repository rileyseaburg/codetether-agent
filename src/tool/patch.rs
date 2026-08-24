//! Apply unified diff patches to workspace files.

#[path = "patch/apply.rs"]
mod apply;
#[path = "patch/approval.rs"]
mod approval;
#[path = "patch/approval_flow.rs"]
mod approval_flow;
#[path = "patch/approval_scope.rs"]
mod approval_scope;
#[path = "patch/approval_scope_bind.rs"]
mod approval_scope_bind;
#[path = "patch/args.rs"]
mod args;
#[path = "patch/backend_policy.rs"]
mod backend_policy;
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
pub(crate) mod proposed;
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

pub use approval_scope::{
    for_root as approval_resource_for_root, from_args as approval_resource_from_args,
    from_patch as approval_resource_from_patch,
};
#[cfg(test)]
include!("patch/test_modules.rs");
