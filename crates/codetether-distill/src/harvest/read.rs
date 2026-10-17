//! Read accepted JSONL records from the SFT directory.
use super::{HarvestFilter, TrainingRecord, parse::parse};
use anyhow::Result;
use std::path::Path;
/// Harvest SFT records from the bus S3 sink (OpenAI-format JSONL).
///
/// # Arguments
/// * `data_dir` - Parent directory containing an `sft` directory of JSONL files.
/// * `filter` - Quality, gate, and record-count limits.
///
/// # Returns
/// Accepted records in directory traversal and line order; missing directories
/// produce an empty result. Malformed or rejected records are skipped.
///
/// # Errors
/// Returns an error when directory entries or file contents cannot be read.
///
/// # Examples
/// ```no_run
/// use codetether_distill::harvest::{HarvestFilter, harvest_from_dir};
/// let records = harvest_from_dir(std::path::Path::new("data"), &HarvestFilter::default())?;
/// assert!(records.len() <= 1000);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn harvest_from_dir(data_dir: &Path, filter: &HarvestFilter) -> Result<Vec<TrainingRecord>> {
    let mut out = Vec::new();
    let dir = data_dir.join("sft");
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.extension().map(|e| e != "jsonl").unwrap_or(true) {
            continue;
        }
        for line in std::fs::read_to_string(&path)?.lines() {
            if out.len() >= filter.max_records {
                break;
            }
            if let Some(r) = parse(line, filter) {
                out.push(r);
            }
        }
    }
    tracing::info!(records = out.len(), "Harvested training records");
    Ok(out)
}
