//! Repository root and shared in-memory cache.

use crate::{Okr, OkrRun};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

/// Repository for OKR persistence operations
///
/// # Examples
/// ```no_run
/// use codetether_okr::{OkrRepository};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let value = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
/// assert!(value.list_okrs().await.unwrap().is_empty());
/// # });
/// ```
pub struct OkrRepository {
    pub(super) base_path: PathBuf,
    pub(super) cache: Arc<RwLock<OkrCache>>,
}

/// In-memory cache for OKR data
#[derive(Debug, Default)]
pub(super) struct OkrCache {
    pub(super) okrs: Vec<Okr>,
    pub(super) runs: Vec<OkrRun>,
}
