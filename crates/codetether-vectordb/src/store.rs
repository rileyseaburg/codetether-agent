//! In-memory vector store with typed payloads and file persistence.

use super::persist;
use super::record::Record;
use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::PathBuf;

/// A file-backed collection of vector [`Record`]s.
#[derive(Debug, Clone, Default)]
pub struct VectorStore<P> {
    pub(super) records: Vec<Record<P>>,
    pub(super) path: Option<PathBuf>,
}

impl<P> VectorStore<P>
where
    P: Serialize + DeserializeOwned + Clone,
{
    /// Create an empty in-memory store (no backing file).
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            path: None,
        }
    }

    /// Open (or initialize) a store backed by `path`.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let records = persist::load(&path)?;
        Ok(Self {
            records,
            path: Some(path),
        })
    }

    /// Number of stored records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Borrow all records.
    pub fn records(&self) -> &[Record<P>] {
        &self.records
    }
}
