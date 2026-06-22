//! Orchestration for `codetether connect`.
//!
//! Spawns the SSH session, streams the remote auth output line-by-line,
//! opens the first verification URL in the local browser, and waits for
//! the remote flow to finish.

use anyhow::{Context, Result, bail};
use tokio::io::{AsyncBufReadExt, BufReader};

use super::{args::ConnectArgs, browser, preflight, ssh, url_scan};

/// Run the full connect → remote device-code → local browser pipeline.
pub async fn execute(args: ConnectArgs) -> Result<()> {
    if !args.skip_preflight {
        preflight::check(&args).await?;
    }
    eprintln!(
        "Connecting to {} (port {}), forwarding {} for auth callback...",
        args.host, args.port, args.forward_port
    );
    let mut child = ssh::build(&args).spawn().context("failed to spawn ssh")?;
    let stdout = child
        .stdout
        .take()
        .context("ssh stdout was not captured")?;
    let mut lines = BufReader::new(stdout).lines();
    let mut opened = false;

    while let Some(line) = lines.next_line().await? {
        println!("{line}");
        if !opened && let Some(url) = url_scan::scan_url(&line) {
            opened = handle_url(&args, &url)?;
        }
    }

    let status = child.wait().await.context("ssh process failed")?;
    if !status.success() {
        bail!(
            "remote `codetether auth {}` failed. If the output shows a shell error \
             like `line 1: -: command not found`, the `{}` binary on the VM is not a \
             valid executable (broken/partial install) — reinstall it there and retry.",
            args.provider,
            args.remote_bin
        );
    }
    Ok(())
}

/// Open the detected URL locally, or print it when `--no-browser` is set.
fn handle_url(args: &ConnectArgs, url: &str) -> Result<bool> {
    if args.no_browser {
        eprintln!("\nVerification URL (open in your browser):\n  {url}\n");
    } else {
        eprintln!("\nOpening verification URL in your local browser...");
        browser::open(url)?;
    }
    Ok(true)
}
