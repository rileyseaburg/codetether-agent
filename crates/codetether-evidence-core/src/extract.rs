//! Collect evidence markers in their established category order.
use super::record::EvidenceRecord;

/// Collect platform, runtime, artifact and database markers in that order.
/// # Arguments
/// `text` is the reported evidence to scan.
/// # Returns
/// Extracted observations; matching markers alone do not establish correctness.
/// # Examples
/// ```
/// let records = codetether_evidence_core::extract::from_text("report.zip");
/// assert_eq!(records[0].kind, "artifact");
/// ```
pub fn from_text(text: &str) -> Vec<EvidenceRecord> {
    let mut out = Vec::new();
    super::extract_platform::collect(text, &mut out);
    super::extract_runtime::collect(text, &mut out);
    super::extract_artifact::collect(text, &mut out);
    super::extract_db::collect(text, &mut out);
    out
}
