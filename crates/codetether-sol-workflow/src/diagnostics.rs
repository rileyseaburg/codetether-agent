//! Collect diagnostics from files touched by successful structured worker tools.
use anyhow::Result;
use codetether_agent_core::ToolUse;
use codetether_validation::{Host, ValidationReport, build_validation_report, track_touched_files};
use serde_json::Value;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
/// Validate only files identified by successful worker tool records.
/// # Arguments
/// `workspace` scopes paths, `tool_uses` supplies the worker history, and `host` collects diagnostics.
/// # Returns
/// A report for the touched files, or `None` when validation produces no report.
/// # Errors
/// Propagates validation failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_validation::Host>(host: &H) -> anyhow::Result<()> {
/// let _report = codetether_sol_workflow::diagnostics::collect(std::path::Path::new("."), &[], host).await?;
/// # Ok(()) }
/// ```
pub async fn collect<H: Host>(
    workspace: &Path,
    tool_uses: &[ToolUse],
    host: &H,
) -> Result<Option<ValidationReport>> {
    let mut touched = HashSet::<PathBuf>::new();
    for tool_use in tool_uses.iter().filter(|tool_use| tool_use.success) {
        let input = serde_json::from_str(&tool_use.input).unwrap_or(Value::Null);
        track_touched_files(&mut touched, workspace, &tool_use.name, &input, None);
    }
    build_validation_report(workspace, &touched, &HashSet::new(), host).await
}
