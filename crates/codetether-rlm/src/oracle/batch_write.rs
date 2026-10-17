//! Write validation buckets and report their paths and counts.

mod buckets;
mod paths;
use super::batch::{BatchValidationStats, SplitWriteStats};
use anyhow::Result;
use buckets::{write_bucket, write_reason_bucket};
use paths::BucketPaths;
use std::path::Path;

impl BatchValidationStats {
    /// Write each bucket to a separate JSONL file.
    ///
    /// Creates `{prefix}.golden.jsonl`, `{prefix}.consensus.jsonl`,
    /// `{prefix}.failed.jsonl`, and `{prefix}.unverified.jsonl`
    /// under `out_dir`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stats = batch.write_jsonl_split("out", "pfx")?;
    /// assert_eq!(stats.golden_count, batch.golden.len());
    /// ```
    ///
    /// # Errors
    /// Returns an error if a directory, file, serialization, or configured storage operation fails.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{oracle::BatchValidationStats};
    /// let directory = tempfile::tempdir().unwrap();
    /// let result = BatchValidationStats::default().write_jsonl_split(directory.path().to_str().unwrap(), "batch").unwrap();
    /// assert_eq!(result.golden_count, 0);
    /// ```
    pub fn write_jsonl_split(&self, out_dir: &str, prefix: &str) -> Result<SplitWriteStats> {
        let dir = Path::new(out_dir);
        std::fs::create_dir_all(dir)?;

        let paths = BucketPaths::new(dir, prefix);
        write_bucket(&paths.golden, &self.golden, "golden")?;
        write_bucket(&paths.consensus, &self.consensus, "consensus")?;
        write_reason_bucket(&paths.failed, &self.failed, "failed")?;
        write_reason_bucket(&paths.unverified, &self.unverified, "unverified")?;

        Ok(paths.into_stats(self))
    }
}
