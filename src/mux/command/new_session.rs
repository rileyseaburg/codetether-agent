//! Spawn and discover a detached mux server process.

use anyhow::Result;
use std::path::PathBuf;

pub(super) async fn run(name: String, directory: Option<PathBuf>, detached: bool) -> Result<()> {
    let workspace = directory.unwrap_or(std::env::current_dir()?);
    let record = crate::mux::control::start_record(&name, workspace).await?;
    if detached {
        println!("started mux session '{}' at {}", name, record.address);
        println!("attach with: codetether mux attach -t {name}");
        Ok(())
    } else {
        crate::mux::client::attach(&record).await
    }
}
