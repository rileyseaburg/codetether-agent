//! Durable local spool for bus S3 training batches.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Durable spool file for one JSONL training batch.
pub(super) struct SpoolFile {
    path: PathBuf,
}

impl SpoolFile {
    /// Atomically persist a JSONL batch before remote upload.
    pub(super) async fn write(key: &str, jsonl: &str) -> Result<Self> {
        let dir = spool_dir().join(key.trim_end_matches(".jsonl"));
        let final_path = dir.with_extension("jsonl");
        let tmp_path = dir.with_extension("jsonl.tmp");
        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&tmp_path, jsonl)
            .await
            .with_context(|| format!("write spool {}", tmp_path.display()))?;
        fs::rename(&tmp_path, &final_path)
            .await
            .with_context(|| format!("commit spool {}", final_path.display()))?;
        Ok(Self { path: final_path })
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) async fn remove(self) -> Result<()> {
        fs::remove_file(&self.path)
            .await
            .with_context(|| format!("remove spool {}", self.path.display()))
    }
}

fn spool_dir() -> PathBuf {
    std::env::var("CODETETHER_TRAINING_SPOOL_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            directories::UserDirs::new()
                .map_or_else(std::env::temp_dir, |dirs| dirs.home_dir().to_path_buf())
                .join(".codetether/training/pending")
        })
}
