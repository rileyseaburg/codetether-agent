//! Worker initialization runtime containers.

use std::{collections::HashSet, sync::Arc};

use tokio::sync::Mutex;

pub(super) type ProcessingSet = Arc<Mutex<HashSet<String>>>;

pub(super) fn processing_set() -> ProcessingSet {
    Arc::new(Mutex::new(HashSet::new()))
}
