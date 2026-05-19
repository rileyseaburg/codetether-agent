//! Runtime-owned refactor guard and optional TetherScript continuation hook.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::validation::ValidationReport;
use types::GuardReport;

mod budget;
mod config;
mod config_defaults;
mod displacement;
mod displacement_similar;
mod git;
mod lines;
mod plugin;
mod plugin_decision;
mod render;
mod rules;
mod scan;
mod similarity;
mod types;
mod wrapper;

pub async fn evaluate(root: &Path, paths: &[PathBuf]) -> Result<Option<ValidationReport>> {
    let files = scan::files(root, paths).await?;
    if files.is_empty() {
        return Ok(None);
    }
    let mut report = GuardReport::new(files);
    rules::apply(&mut report);
    if let Some(violation) = plugin::apply(root, &report).await? {
        report.violations.push(violation);
    }
    if report.violations.is_empty() {
        return Ok(None);
    }
    Ok(Some(ValidationReport {
        issue_count: report.violations.len(),
        prompt: render::prompt(&report),
    }))
}

#[cfg(test)]
mod tests;
