//! Create a mux session, joining the workspace's server or starting one.

use anyhow::Result;
use std::path::PathBuf;

pub(super) async fn run(
    name: String,
    directory: Option<PathBuf>,
    start: crate::cli::command::mux_args::MuxStartOptions,
) -> Result<()> {
    let workspace = directory.unwrap_or(std::env::current_dir()?);
    let isolation = crate::mux::isolation::Isolation::from_no_worktree(start.no_worktree);
    let target = crate::mux::control::start_target(&name, workspace, isolation).await?;
    if start.detached {
        println!(
            "started mux session '{}' at {}",
            name, target.record.address
        );
        println!("attach with: codetether mux attach {name}");
        Ok(())
    } else {
        crate::mux::client::attach(&target).await
    }
}
