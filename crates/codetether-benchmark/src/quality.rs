//! Sequential command execution and result capture for benchmark quality gates.
use crate::QualityCheckResult;
use std::{path::Path, process::Command};
/// Execute each configured command in order; skip entries whose command is absent.
///
/// # Arguments
/// * `working_dir` - Current directory supplied to every child shell.
/// * `checks` - Ordered pairs of result names and optional shell commands.
/// # Returns
/// One result per configured command, including launch failures and failed output.
/// # Examples
/// ```
/// let checks = [("disabled", &None)];
/// assert!(codetether_benchmark::quality::run_quality_checks(std::path::Path::new("."), &checks).is_empty());
/// ```

/// Run quality checks in a directory and collect results
pub fn run_quality_checks(
    working_dir: &Path,
    checks: &[(&str, &Option<String>)],
) -> Vec<QualityCheckResult> {
    let mut results = Vec::new();

    for &(name, cmd) in checks {
        if let Some(command) = cmd {
            let output = Command::new("/bin/sh")
                .arg("-c")
                .arg(command)
                .current_dir(working_dir)
                .output();

            let (passed, output_text) = match output {
                Ok(o) => {
                    let passed = o.status.success();
                    let text = if passed {
                        None
                    } else {
                        let stderr = String::from_utf8_lossy(&o.stderr);
                        let stdout = String::from_utf8_lossy(&o.stdout);
                        Some(format!("{}\n{}", stdout, stderr))
                    };
                    (passed, text)
                }
                Err(e) => (false, Some(format!("Failed to execute: {}", e))),
            };

            results.push(QualityCheckResult {
                name: name.to_string(),
                passed,
                output: output_text,
            });
        }
    }

    results
}
