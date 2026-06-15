//! `worktree open` handler.

use anyhow::anyhow;
use codetether_agent::{cli::WorktreeOpenArgs, worktree::WorktreeManager};

/// Open a named worktree in VS Code.
pub async fn run(args: WorktreeOpenArgs, mgr: &WorktreeManager) -> anyhow::Result<()> {
    let worktrees = mgr.list().await;
    let info = worktrees
        .iter()
        .find(|w| w.name == args.name)
        .ok_or_else(|| {
            let names: Vec<&str> = worktrees.iter().map(|w| w.name.as_str()).collect();
            anyhow!(
                "Worktree '{}' not found. Available: {}",
                args.name,
                if names.is_empty() {
                    "(none)".to_string()
                } else {
                    names.join(", ")
                }
            )
        })?;
    mgr.open_in_vscode(info, args.reuse_window).await?;
    println!("Opened worktree '{}' in VS Code.", info.name);
    Ok(())
}
