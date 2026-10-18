//! Orchestrate scanning, built-in rules, extension hooks and rendering.
use crate::{GuardReport, Hook, Report};
use crate::{render, rules, scan};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Scan source paths and report built-in and native-hook violations.
/// # Arguments
/// Receives a workspace root, changed file paths and an optional extension hook.
/// # Returns
/// None when no configured files or violations remain; otherwise continuation instructions.
/// # Errors
/// Propagates source-file read and native-hook failures.
/// # Examples
/// ```no_run
/// # async fn example(hook: &impl codetether_refactor_guard::Hook) -> anyhow::Result<()> {
/// let report = codetether_refactor_guard::evaluate(std::path::Path::new("."), &[], hook).await?;
/// assert!(report.is_none());
/// # Ok(())
/// # }
/// ```
pub async fn evaluate<H: Hook>(root: &Path, paths: &[PathBuf], hook: &H) -> Result<Option<Report>> {
    let files = scan::files(root, paths).await?;
    if files.is_empty() {
        return Ok(None);
    }
    let mut report = GuardReport::new(files);
    rules::apply(&mut report);
    if let Some(violation) = hook.apply(root, &report).await? {
        report.violations.push(violation);
    }
    if report.violations.is_empty() {
        return Ok(None);
    }
    Ok(Some(Report {
        issue_count: report.violations.len(),
        prompt: render::prompt(&report),
    }))
}
