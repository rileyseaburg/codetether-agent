//! Validate shell output and created artifacts after host execution.
use super::ShellExecutor;
use super::file::glob_exists;
use anyhow::{Result, bail};
use std::path::Path;

pub async fn run(
    root: &Path,
    command: &str,
    cwd: &Option<String>,
    expect_output_contains: &[String],
    expect_files_glob: &[String],
    executor: &dyn ShellExecutor,
) -> Result<()> {
    let dir = cwd
        .as_ref()
        .map_or_else(|| root.to_path_buf(), |c| root.join(c));
    let result = executor.execute(&dir, command).await?;
    let combined = result.output;

    if !result.success {
        bail!("command `{command}` failed: {combined}");
    }
    for expected in expect_output_contains {
        if !combined.contains(expected) {
            bail!("command output did not contain `{expected}`");
        }
    }
    for pattern in expect_files_glob {
        if !glob_exists(&dir, pattern)? {
            bail!("command did not create files matching `{pattern}`");
        }
    }
    Ok(())
}
