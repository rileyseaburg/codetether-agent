//! Append-only JSONL implementation of [`DurableLog`].
//!
//! One file per partition under a base directory. Each line is a JSON
//! [`BusEnvelope`]; the line's 1-based ordinal is its offset. This is the
//! zero-dependency default backend, reusing the JSONL record shape already
//! produced by [`crate::bus::s3_sink`].

use super::durable_log::{DurableLog, partition_of};
use super::BusEnvelope;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

mod file_path;
mod file_read;

/// File-backed durable log rooted at `dir`.
pub struct FileDurableLog {
    dir: PathBuf,
    write_lock: Mutex<()>,
}

impl FileDurableLog {
    /// Create a log rooted at `dir` (created on first append).
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            write_lock: Mutex::new(()),
        }
    }
}

#[async_trait]
impl DurableLog for FileDurableLog {
    async fn append(&self, env: &BusEnvelope) -> Result<u64> {
        let path = file_path::partition_path(&self.dir, partition_of(env));
        let mut line = serde_json::to_string(env).context("serialize envelope")?;
        line.push('\n');
        let _guard = self.write_lock.lock().await;
        fs::create_dir_all(&self.dir).await.ok();
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await
            .with_context(|| format!("open {}", path.display()))?;
        f.write_all(line.as_bytes()).await.context("append line")?;
        file_read::count_lines(&path).await
    }

    async fn tail(&self, partition: &str, after: u64) -> Result<Vec<BusEnvelope>> {
        let path = file_path::partition_path(&self.dir, partition);
        file_read::read_after(&path, after).await
    }
}
