//! Create a mux session, joining the workspace's server or starting one.

use anyhow::Result;
use std::path::PathBuf;

pub(super) async fn run(
    name: String,
    directory: Option<PathBuf>,
    start: crate::MuxStartOptions,
    host: &impl crate::Host,
) -> Result<()> {
    let workspace = directory.unwrap_or(std::env::current_dir()?);
    let isolation = codetether_mux_workspace::Isolation::from_no_worktree(start.no_worktree);
    let target = codetether_mux_lifecycle::start_target(&name, workspace, isolation).await?;
    if start.detached {
        println!(
            "started mux session '{}' at {}",
            name, target.record.address
        );
        println!("attach with: codetether mux attach {name}");
        Ok(())
    } else {
        crate::attachment::run(&target, host).await
    }
}
