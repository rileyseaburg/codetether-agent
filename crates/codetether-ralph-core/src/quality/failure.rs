//! Summarize failed-check output with the original bounded logging.
use std::path::Path;
use tracing::warn;
pub(super) fn report_failure(name: &str, cargo_root: &Path, output: &str) {
    let error_summary: String = output
        .lines()
        .filter(|line| {
            line.starts_with("error") || line.contains("error:") || line.contains("error[")
        })
        .take(5)
        .collect::<Vec<_>>()
        .join("\n");
    warn!(
        check = %name,
        cargo_root = %cargo_root.display(),
        error_summary = %error_summary.chars().take(300).collect::<String>(),
        "{} check failed in {:?}",
        name,
        cargo_root
    );
}
