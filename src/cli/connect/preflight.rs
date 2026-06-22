//! Remote binary preflight for `codetether connect`.
//!
//! Before forwarding ports and starting the device-code flow, verify the
//! remote `codetether` binary actually exists and is executable. A broken
//! install surfaces as a shell error (`line 1: -: command not found`), so we
//! probe with `command -v`, `file`, and `--version` and report concretely.

use super::args::ConnectArgs;
use super::ssh;
use anyhow::{Context, Result, bail};

/// Probe the remote binary; bail with a precise hint if it is not runnable.
pub async fn check(args: &ConnectArgs) -> Result<()> {
    let bin = &args.remote_bin;
    let probe = format!(
        "p=$(command -v {bin} 2>/dev/null) || {{ echo MISSING; exit 0; }}; \
         echo PATH=$p; file -L \"$p\" 2>/dev/null || true; \
         \"$p\" --version 2>&1 | head -n1 || echo VERSION_FAILED"
    );
    let mut cmd = ssh::probe_command(args, &probe);
    let out = cmd.output().await.context("failed to run ssh preflight")?;
    let report = String::from_utf8_lossy(&out.stdout);
    eprintln!("Preflight: {}", report.trim().replace('\n', " | "));
    if report.contains("MISSING") {
        bail!("`{bin}` not found on {}. Install it on the VM first.", args.host);
    }
    if !report.contains("ELF") && report.contains("text") {
        bail!(
            "`{bin}` on {} is a text file, not an executable (broken/partial \
             install). Reinstall the correct linux binary and retry.",
            args.host
        );
    }
    Ok(())
}
