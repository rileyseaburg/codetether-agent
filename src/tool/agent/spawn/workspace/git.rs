//! Checked Git queries used to identify a child's branch base and storage root.

use anyhow::{Context, Result, ensure};
use std::path::Path;

pub(super) async fn output(cwd: &Path, args: &[&str]) -> Result<String> {
    let output = tokio::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .context("Query child workspace Git identity")?;
    ensure!(
        output.status.success(),
        "Git identity query failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
