//! Cross-process serialization for approval decision transactions.

use super::ApprovalStore;
use anyhow::Result;
use fs2::FileExt;
use std::fs::{File, OpenOptions};

pub(crate) struct DecisionLock(File);

impl ApprovalStore {
    pub(crate) fn lock_decisions(&self) -> Result<DecisionLock> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.root.join("decisions.lock"))?;
        file.lock_exclusive()?;
        Ok(DecisionLock(file))
    }
}

impl Drop for DecisionLock {
    fn drop(&mut self) {
        if let Err(error) = FileExt::unlock(&self.0) {
            tracing::warn!(%error, "Failed to unlock approval decision store");
        }
    }
}
