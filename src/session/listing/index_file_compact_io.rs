//! Blocking I/O for the listing index compact-rewrite.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

use super::summary::SessionSummary;

/// Synchronous tmp-file + atomic rename writer. Holds the index writer
/// lock so a concurrent save's append cannot land on the unlinked
/// pre-rename inode.
pub(super) fn write_compact_sync(final_path: &Path, summaries: &[SessionSummary]) -> Result<()> {
    let _guard = super::index_file_io::write_lock();
    if let Some(parent) = final_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let tmp = final_path.with_extension("jsonl.tmp");
    let mut file = File::create(&tmp)
        .with_context(|| format!("create tmp index {}", tmp.display()))?;
    for s in summaries {
        let mut line = serde_json::to_vec(s)?;
        line.push(b'\n');
        file.write_all(&line)?;
    }
    file.flush()?;
    match std::fs::rename(&tmp, final_path) {
        Ok(()) => Ok(()),
        Err(primary) => {
            let _ = std::fs::remove_file(final_path);
            if let Err(retry) = std::fs::rename(&tmp, final_path) {
                let _ = std::fs::remove_file(&tmp);
                anyhow::bail!(
                    "rename tmp into {}: {primary} (retry: {retry})",
                    final_path.display()
                );
            }
            Ok(())
        }
    }
}
