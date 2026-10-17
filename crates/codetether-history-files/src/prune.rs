//! Remove stale regular files after the full transcript has been materialized.
use anyhow::{Context, Result};
use std::{collections::HashSet, ffi::OsString, path::Path};
use tokio::fs;

pub(super) async fn prune_stale_turn_files(dir: &Path, expected: &HashSet<OsString>) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .await
        .with_context(|| format!("read_dir {}", dir.display()))?;
    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        if !file_type.is_file() {
            continue;
        }
        let name = entry.file_name();
        if !expected.contains(&name) {
            fs::remove_file(entry.path())
                .await
                .with_context(|| format!("remove {}", entry.path().display()))?;
        }
    }
    Ok(())
}

pub(super) fn file_name(path: &Path) -> Result<OsString> {
    path.file_name()
        .map(|name| name.to_os_string())
        .context("materialized history path missing file name")
}
