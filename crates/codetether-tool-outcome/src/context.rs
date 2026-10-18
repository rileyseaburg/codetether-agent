//! Borrowed workspace and call information for processing one result.
use serde_json::Value;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
/// Supplies original call input and workspace state for result classification.
/// # Examples
/// ```
/// let mut touched = std::collections::HashSet::new();
/// let input = serde_json::json!({"path": "file.txt"});
/// let context = codetether_tool_outcome::Context {
///     tool: "read", input: &input, cwd: std::path::Path::new("."),
///     touched: &mut touched, auto_apply_edits: false,
/// };
/// assert_eq!(context.tool, "read");
/// ```
pub struct Context<'a> {
    /// Canonical tool name.
    pub tool: &'a str,
    /// Original normalized input, before runtime enrichment.
    pub input: &'a Value,
    /// Workspace root used to resolve touched paths.
    pub cwd: &'a Path,
    /// Paths accumulated for post-turn validation.
    pub touched: &'a mut HashSet<PathBuf>,
    /// Session opt-in; environment policy is evaluated only for pending edits.
    pub auto_apply_edits: bool,
}
