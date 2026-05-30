//! Benchmark tool profile: optionally restrict the tool surface.
//!
//! A large tool surface bloats the request context and degrades tool-selection
//! accuracy. For SWE-bench / Terminal-Bench runs, set
//! `CODETETHER_TOOL_PROFILE=lean` to expose only the core coding tools.

use crate::provider::ToolDefinition;

/// Core tool IDs kept in the `lean` profile.
const LEAN_TOOL_IDS: &[&str] = &[
    "read",
    "write",
    "edit",
    "multiedit",
    "bash",
    "grep",
    "glob",
    "list",
    "tree",
    "codesearch",
    "fileinfo",
    "headtail",
    "diff",
    "lsp",
    "patch",
    "todo",
];

/// Returns `true` when the lean benchmark profile is active.
pub fn is_lean() -> bool {
    std::env::var("CODETETHER_TOOL_PROFILE")
        .map(|v| v.eq_ignore_ascii_case("lean"))
        .unwrap_or(false)
}

/// Filter tool definitions according to the active profile.
///
/// When the lean profile is inactive, definitions pass through unchanged.
pub fn apply(defs: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    if !is_lean() {
        return defs;
    }
    defs.into_iter()
        .filter(|d| LEAN_TOOL_IDS.contains(&d.name.as_str()))
        .collect()
}
