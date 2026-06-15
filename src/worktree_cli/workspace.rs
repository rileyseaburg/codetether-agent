//! `worktree workspace` handler.

use codetether_agent::{cli::WorktreeWorkspaceArgs, worktree::WorktreeManager};

/// Write a multi-root `.code-workspace` file, optionally opening it.
pub async fn run(args: WorktreeWorkspaceArgs, mgr: &WorktreeManager) -> anyhow::Result<()> {
    let worktrees = mgr.list().await;
    let written = mgr.write_code_workspace(&worktrees, &args.dest).await?;
    println!(
        "Wrote workspace with {} worktree(s) to {}",
        worktrees.len(),
        written.display()
    );
    if args.open {
        mgr.open_workspace_in_vscode(&written).await?;
        println!("Opened workspace in VS Code.");
    }
    Ok(())
}
