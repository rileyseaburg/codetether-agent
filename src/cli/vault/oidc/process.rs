//! Delegate browser OIDC to Vault CLI without modifying its token helper.

use anyhow::{Context, Result, ensure};
use std::process::Stdio;

pub(super) async fn run(
    address: &str,
    mount: &str,
    role: &str,
    no_browser: bool,
) -> Result<String> {
    let executable = which::which("vault")
        .context("Browser OIDC requires Vault CLI on PATH; install it or use token/device login")?;
    let mut child = tokio::process::Command::new(executable)
        .args(["login", "-method=oidc", "-no-store", "-format=json"])
        .arg(format!("-path={mount}"))
        .arg(format!("role={role}"))
        .arg(format!("skip_browser={no_browser}"))
        .env("VAULT_ADDR", address)
        .env_remove("VAULT_TOKEN")
        .env_remove("VAULT_FORMAT")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("Could not start Vault OIDC login")?;
    let stdout = child.stdout.take().context("Missing Vault CLI output")?;
    let stderr = child
        .stderr
        .take()
        .context("Missing Vault CLI diagnostics")?;
    let output = super::stream::collect(stdout, true);
    let diagnostics = super::stream::collect(stderr, false);
    let (status, output, _) = tokio::try_join!(
        async { child.wait().await.map_err(anyhow::Error::from) },
        output,
        diagnostics
    )?;
    ensure!(
        status.success(),
        "Vault OIDC login failed; check the configured non-admin role and SSH callback forwarding"
    );
    Ok(output)
}
