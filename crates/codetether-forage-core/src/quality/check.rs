//! Execute and format one quality check through a host-supplied runner.
use super::{QualityShellOutput, spec::CheckSpec};
use anyhow::Result;
use std::future::Future;
pub(super) async fn run<F, Fut>(
    spec: &CheckSpec,
    execute: &mut F,
    results: &mut Vec<String>,
    all_passed: &mut bool,
) where
    F: FnMut(&'static str) -> Fut,
    Fut: Future<Output = Result<QualityShellOutput>>,
{
    match execute(spec.command).await {
        Ok(output) => {
            let status = if output.success {
                "PASS"
            } else {
                *all_passed = false;
                "FAIL"
            };
            let summary = if output.combined.chars().count() > spec.limit {
                crate::truncate_chars_with_suffix(&output.combined, spec.limit, " [...truncated]")
            } else if output.combined.is_empty() {
                spec.empty.to_string()
            } else {
                output.combined
            };
            results.push(format!("{}: {} - {}", spec.label, status, summary));
        }
        Err(e) => {
            results.push(format!("{}: ERROR - {}", spec.label, e));
            *all_passed = false;
        }
    }
}
