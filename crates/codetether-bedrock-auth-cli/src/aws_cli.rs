//! AWS CLI invocation helpers for Bedrock auth.

use super::exported;
use anyhow::{Context, Result, bail};

/// Shell out to `aws configure export-credentials`.
pub(super) async fn export_credentials(profile: Option<&str>) -> Result<exported::Exported> {
    let mut cmd = tokio::process::Command::new("aws");
    cmd.args(["configure", "export-credentials", "--format", "process"]);
    if let Some(p) = profile {
        eprintln!("Exporting AWS credentials from profile '{p}'.");
        cmd.args(["--profile", p]);
    }
    let out = cmd
        .output()
        .await
        .context("Failed to run AWS CLI v2 export-credentials")?;
    if !out.status.success() {
        bail!(
            "AWS CLI could not export credentials: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    exported::parse(&out.stdout)
}
