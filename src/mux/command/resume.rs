//! Resume a durable CodeTether session inside a fresh mux session.
//!
//! `codetether mux resume --session <id>` differs from `mux new` in two ways: the
//! workspace comes from the session snapshot rather than the caller's current
//! directory, and the launched TUI is told to reopen that session. Resolving the
//! workspace first means a bad session ID fails before a server or worktree is
//! created.

use anyhow::{Context, Result};

pub(super) async fn run(session_id: String, name: Option<String>, detached: bool) -> Result<()> {
    let workspace = crate::session::Session::recorded_workspace(&session_id)
        .await
        .context("resolve workspace for the requested session")?;
    let name = name.unwrap_or_else(|| crate::mux::resume_name::derive(&session_id));
    let summary = crate::mux::control::start_managed_session(&name, workspace, Some(&session_id))
        .await
        .context("start mux session for resume")?;
    if detached {
        println!(
            "resumed session {session_id} in mux '{name}' at {}",
            summary.address
        );
        println!("attach with: codetether mux attach {name}");
        return Ok(());
    }
    let record = crate::mux::registry::load(&name).await?;
    crate::mux::client::attach(&record).await
}
