//! JSONL split-write implementation for batch stats.
//!
//! Writes each validation bucket (golden, consensus, failed,
//! unverified) to a separate `.jsonl` file under a given
//! output directory.
//!
//! # Examples
//!
//! ```ignore
//! let stats = batch.write_jsonl_split("output", "prefix")?;
//! ```

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

use super::batch::{BatchValidationStats, SplitWriteStats};
use super::record::OracleTraceRecord;

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

struct BucketPaths {
    golden: std::path::PathBuf,
    consensus: std::path::PathBuf,
    failed: std::path::PathBuf,
    unverified: std::path::PathBuf,
}

impl BucketPaths {
    fn new(dir: &Path, prefix: &str) -> Self {
        Self {
            golden: dir.join(format!("{prefix}.golden.jsonl")),
            consensus: dir.join(format!("{prefix}.consensus.jsonl")),
            failed: dir.join(format!("{prefix}.failed.jsonl")),
            unverified: dir.join(format!("{prefix}.unverified.jsonl")),
        }
    }

    fn into_stats(self, b: &BatchValidationStats) -> SplitWriteStats {
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

fn write_bucket(
    path: &Path,
    traces: &[super::trace_types::ValidatedTrace],
    verdict: &str,
) -> Result<()> {
    let mut w = BufWriter::new(File::create(path)?);
    for trace in traces {
        let rec = OracleTraceRecord {
            verdict: verdict.to_string(),
            reason: None,
            agreement_ratio: None,
            trace: trace.clone(),
        };
        writeln!(w, "{}", serde_json::to_string(&rec)?)?;
    }
    w.flush()?;
    Ok(())
}

fn write_reason_bucket(
    path: &Path,
    items: &[(super::trace_types::ValidatedTrace, String)],
    verdict: &str,
) -> Result<()> {
    let mut w = BufWriter::new(File::create(path)?);
    for (trace, reason) in items {
        let rec = OracleTraceRecord {
            verdict: verdict.to_string(),
            reason: Some(reason.clone()),
            agreement_ratio: None,
            trace: trace.clone(),
        };
        writeln!(w, "{}", serde_json::to_string(&rec)?)?;
    }
    w.flush()?;
    Ok(())
}
