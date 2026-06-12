//! Blocking I/O for the listing index compact-rewrite.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

use super::summary::SessionSummary;

/// Synchronous tmp-file + atomic rename writer.
pub(super) fn write_compact_sync(final_path: &Path, summaries: &[SessionSummary]) -> Result<()> {
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
        Err(_) => {
            let _ = std::fs::remove_file(final_path);
            std::fs::rename(&tmp, final_path)
                .with_context(|| format!("rename tmp into {}", final_path.display()))
        }
    }
}
