use anyhow::{Context, Result, ensure};
use std::{path::Path, process::Stdio};
use tokio::process::Command;

pub(crate) fn command(program: &str, args: &[String], cwd: &Path) -> Command {
    let mut command = Command::new(program);
    command.args(args).current_dir(cwd).kill_on_drop(true);
    command.env("RUSTUP_AUTO_INSTALL", "0");
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
}

pub(crate) async fn execute(program: &str, args: &[String], cwd: &Path) -> Result<()> {
    let status = command(program, args, cwd)
        .status()
        .await
        .with_context(|| format!("could not execute {program}"))?;
    ensure!(status.success(), "{program} exited with {status}");
    Ok(())
}

pub(crate) async fn capture(program: &str, args: &[String], cwd: &Path) -> Result<String> {
    let output = command(program, args, cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("could not execute {program}"))?
        .wait_with_output()
        .await?;
    ensure!(
        output.status.success(),
        "{program} exited with {}",
        output.status
    );
    String::from_utf8(output.stdout).context("command output was not UTF-8")
}
