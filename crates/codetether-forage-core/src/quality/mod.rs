//! Ordered quality checks with host-owned command execution and authorization.
use anyhow::Result;
use std::future::Future;
mod check;
mod output;
mod spec;
pub use output::QualityShellOutput;
/// Execute the existing ordered quality checks through a host runner.
///
/// # Arguments
/// * `changed_files` — Files used to determine whether Rust checks apply.
/// * `execute` — Host callback responsible for authorization and command execution.
/// # Returns
/// Combined check summaries and whether every check passed; non-Rust changes skip checks.
/// # Errors
/// Runner failures are included in the summary and mark the overall result false.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_forage_core::quality::{run, QualityShellOutput};
/// let result = run(&[], |_| std::future::ready(Ok(QualityShellOutput { success: true, combined: String::new() }))).await?;
/// assert!(result.1);
/// # Ok(()) }
/// ```
pub async fn run<F, Fut>(changed_files: &[String], mut execute: F) -> Result<(String, bool)>
where
    F: FnMut(&'static str) -> Fut,
    Fut: Future<Output = Result<QualityShellOutput>>,
{
    let has_rust_files = changed_files.iter().any(|f| f.ends_with(".rs"));
    if !has_rust_files {
        return Ok((
            "no Rust files changed, skipping quality gates".to_string(),
            true,
        ));
    }
    let mut results = Vec::new();
    let mut all_passed = true;
    for spec in &spec::CHECKS {
        check::run(spec, &mut execute, &mut results, &mut all_passed).await;
    }
    Ok((results.join("\n"), all_passed))
}
