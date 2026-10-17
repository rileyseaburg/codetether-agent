//! Atomic publication of newly generated runtime identities.

use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub(super) fn publish(path: &Path, identity: &str) -> Result<bool> {
    let temp = path.with_extension(format!("tmp-{}", Uuid::new_v4().simple()));
    let result = publish_from_temp(path, &temp, identity);
    let _ = std::fs::remove_file(temp);
    result
}

fn publish_from_temp(path: &Path, temp: &Path, identity: &str) -> Result<bool> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp)
        .context("Failed to create temporary A2A identity")?;
    writeln!(file, "{identity}").context("Failed to write temporary A2A identity")?;
    file.sync_all()
        .context("Failed to sync temporary A2A identity")?;
    match std::fs::hard_link(temp, path) {
        Ok(()) => {
            sync_parent(path)?;
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(error) => Err(error).context("Failed to publish A2A identity"),
    }
}

#[cfg(unix)]
fn sync_parent(path: &Path) -> Result<()> {
    let parent = path.parent().context("A2A identity path has no parent")?;
    std::fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .context("Failed to sync A2A identity directory")
}

#[cfg(not(unix))]
fn sync_parent(_path: &Path) -> Result<()> {
    Ok(())
}
