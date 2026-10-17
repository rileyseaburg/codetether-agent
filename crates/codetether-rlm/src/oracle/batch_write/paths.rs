//! Bucket file names and persisted counts.

use crate::oracle::batch::{BatchValidationStats, SplitWriteStats};
use std::path::Path;

pub(super) struct BucketPaths {
    pub(super) golden: std::path::PathBuf,
    pub(super) consensus: std::path::PathBuf,
    pub(super) failed: std::path::PathBuf,
    pub(super) unverified: std::path::PathBuf,
}

impl BucketPaths {
    pub(super) fn new(dir: &Path, prefix: &str) -> Self {
        Self {
            golden: dir.join(format!("{prefix}.golden.jsonl")),
            consensus: dir.join(format!("{prefix}.consensus.jsonl")),
            failed: dir.join(format!("{prefix}.failed.jsonl")),
            unverified: dir.join(format!("{prefix}.unverified.jsonl")),
        }
    }

    pub(super) fn into_stats(self, b: &BatchValidationStats) -> SplitWriteStats {
        SplitWriteStats {
            golden_path: self.golden.to_string_lossy().into(),
            consensus_path: self.consensus.to_string_lossy().into(),
            failed_path: self.failed.to_string_lossy().into(),
            unverified_path: self.unverified.to_string_lossy().into(),
            golden_count: b.golden.len(),
            consensus_count: b.consensus.len(),
            failed_count: b.failed.len(),
            unverified_count: b.unverified.len(),
        }
    }
}
