//! Append-only JSONL implementation of [`DurableLog`].
//!
//! One file per partition under a base directory. Each line is a JSON
//! [`BusEnvelope`]; the line's 1-based ordinal is its offset. This is the
//! default file backend, storing the serialized [`BusEnvelope`] record shape.

use super::BusEnvelope;

use super::durable_log::{DurableLog, partition_of};

use anyhow::{Context, Result};

use async_trait::async_trait;

use std::path::PathBuf;

use tokio::fs::{self, OpenOptions};

use tokio::io::AsyncWriteExt;

use tokio::sync::Mutex;

mod file_path;

mod file_read;
mod operations;

/// File-backed durable log rooted at `dir`.
///
/// # Examples
/// ```
/// let log = codetether_bus_core::durable_log_file::FileDurableLog::new("logs");
/// ```
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
