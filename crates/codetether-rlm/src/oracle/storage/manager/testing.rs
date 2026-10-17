//! Dependency injection for local storage tests.

use super::super::remote::OracleRemote;
use super::OracleTraceStorage;
use std::{path::PathBuf, sync::Arc};
impl OracleTraceStorage {
    /// Construct with injected dependencies for testing.
    #[cfg(test)]
    pub(in crate::oracle) fn new_for_test(
        spool_dir: PathBuf,
        max_spool_bytes: u64,
        remote: Option<Arc<dyn OracleRemote>>,
    ) -> Self {
        Self {
            spool_dir,
            max_spool_bytes,
            remote,
        }
    }
}
