//! Atomic, synchronized sidecar replacement after canonical metadata commits.
use crate::RunCheckpoint;
use std::{io::Write, path::Path};
pub(super) async fn replace(path: &Path, checkpoint: &RunCheckpoint) -> anyhow::Result<()> {
    let bytes = serde_json::to_vec_pretty(checkpoint)?;
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || {
        let parent = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("checkpoint path needs a parent"))?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        temporary.persist(&path).map_err(|error| error.error)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        anyhow::Ok(())
    })
    .await?
}
