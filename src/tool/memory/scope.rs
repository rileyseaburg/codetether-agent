//! Stable scope resolution for the memory tool.

use serde_json::Value;
use std::path::PathBuf;

pub(super) fn save(args: &Value) -> Option<String> {
    explicit(args).or_else(|| stable(args))
}

pub(super) fn search(args: &Value) -> Option<String> {
    let scope = explicit(args);
    match scope.as_deref() {
        Some("all") => None,
        _ => scope.or_else(|| stable(args)),
    }
}

fn explicit(args: &Value) -> Option<String> {
    let s = args["scope"].as_str()?.trim();
    (!s.is_empty() && s != "current project" && s != "current_project").then(|| s.to_string())
}

fn stable(args: &Value) -> Option<String> {
    let cwd = args["__ct_parent_workspace"]
        .as_str()
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())?;
    super::git_scope::remote(&cwd)
        .or_else(|| super::git_scope::common(&cwd))
        .or_else(|| super::git_scope::path_scope("path", &cwd))
}
