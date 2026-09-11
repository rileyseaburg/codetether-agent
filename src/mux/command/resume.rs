//! Resume a durable CodeTether session inside a fresh mux session.
//!
//! `codetether mux resume --session <id>` differs from `mux new` in two ways: the
//! workspace comes from the session snapshot rather than the caller's current
//! directory, and the launched TUI is told to reopen that session. Resolving the
//! workspace first means a bad session ID fails before a server or worktree is
//! created.

use anyhow::{Context, Result};

pub(super) async fn run(
    session_id: String,
    name: Option<String>,
    start: crate::cli::command::mux_args::MuxStartOptions,
) -> Result<()> {
    let workspace = crate::session::Session::recorded_workspace(&session_id)
        .await
        .context("resolve workspace for the requested session")?;
    let name = name.unwrap_or_else(|| crate::mux::resume_name::derive(&session_id));
    let isolation = crate::mux::isolation::Isolation::from_no_worktree(start.no_worktree);
    let summary =
        crate::mux::control::start_managed_session(&name, workspace, Some(&session_id), isolation)
            .await
            .context("start mux session for resume")?;
    if start.detached {
        println!(
            "resumed session {session_id} in mux '{name}' at {}",
            summary.address
        );
        println!("attach with: codetether mux attach {name}");
        return Ok(());
    }
    let target = crate::mux::registry::load(&name).await?;
    crate::mux::client::attach(&target).await
}
