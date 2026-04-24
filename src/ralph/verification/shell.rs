use super::file::glob_exists;
use anyhow::{Result, bail};
use std::path::Path;
use std::process::Command;

pub fn run(
    root: &Path,
    command: &str,
    cwd: &Option<String>,
    expect_output_contains: &[String],
    expect_files_glob: &[String],
) -> Result<()> {
    let dir = cwd
        .as_ref()
        .map_or_else(|| root.to_path_buf(), |c| root.join(c));
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .current_dir(&dir)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");

    if !output.status.success() {
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
