//! Safe direct process construction for a mux agent turn.

use std::path::Path;
use std::process::Stdio;

use anyhow::Result;

pub(super) fn child(
    prompt: &str,
    session_id: Option<&str>,
    max_steps: usize,
    tool_profile: Option<&str>,
    workspace: &Path,
    mux: &str,
) -> Result<tokio::process::Child> {
    let mut command = super::executable::command()?;
    command.args([
        "run",
        "--format",
        "jsonl",
        "--max-steps",
        &max_steps.to_string(),
    ]);
    if let Some(session_id) = session_id {
        command.args(["--session", session_id]);
    }
    command.arg(prompt).arg("--").arg(workspace);
    command.current_dir(workspace);
    command.env(crate::mux::coordination::SESSION_ENV, mux);
    if let Some(profile) = tool_profile {
        command.env("CODETETHER_TOOL_PROFILE", profile);
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    super::spawn_group::isolate(&mut command);
    command.kill_on_drop(true);
    Ok(command.spawn()?)
}
