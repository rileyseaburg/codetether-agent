//! Worker initialization runtime containers.

use std::{collections::HashSet, sync::Arc};

use tokio::sync::Mutex;

/// Shared set of task identifiers currently being processed.
pub type ProcessingSet = Arc<Mutex<HashSet<String>>>;

/// Create an empty shared processing set.
/// # Returns
/// A reference-counted, asynchronously locked set.
/// # Examples
/// ```
/// let set = codetether_a2a_worker_core::initialization::processing_set();
/// assert!(set.try_lock().unwrap().is_empty());
/// ```
pub fn processing_set() -> ProcessingSet {
    Arc::new(Mutex::new(HashSet::new()))
}
