//! Workspace- and content-bound patch approval resources.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

#[path = "approval_scope_mode.rs"]
mod mode;
#[path = "approval_scope_path.rs"]
mod path;

/// Return the approval resource for a patch rooted at the current directory.
pub fn from_patch(patch: &str) -> String {
    for_root(&path::current(), patch)
}

/// Return the approval resource for a patch rooted at `root`.
pub fn for_root(root: &Path, patch: &str) -> String {
    mode::apply(base(root, patch))
}

/// Return the approval resource represented by complete invocation arguments.
pub fn from_args(args: &Value) -> String {
    let patch = args
        .get("patch")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let root = crate::tool::network_access::trusted_workspace(args)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(path::current);
    mode::for_args(base(&root, patch), args)
}

fn base(root: &Path, patch: &str) -> String {
    let hunks = super::parser::parse_patch(patch);
    build(root, &super::group::files(&hunks), patch)
}

pub(super) fn for_apply(root: &Path, files: &[String], patch: &str) -> String {
    mode::apply(build(root, files, patch))
}

pub(super) fn for_mode(root: &Path, files: &[String], patch: &str, preview: bool) -> String {
    mode::select(build(root, files, patch), preview)
}

pub(super) fn build(root: &Path, files: &[String], patch: &str) -> String {
    let paths = match files {
        [] => "workspace".to_string(),
        [file] => file.clone(),
        many => many.join(","),
    };
    format!(
        "root={}::{paths}#sha256={}",
        path::absolute(root).display(),
        hex::encode(Sha256::digest(patch.as_bytes()))
    )
}
