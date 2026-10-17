//! Serialize trace records into JSONL buckets.

use crate::oracle::record::OracleTraceRecord;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub(super) fn write_bucket(
    path: &Path,
    traces: &[crate::oracle::trace_types::ValidatedTrace],
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

pub(super) fn write_reason_bucket(
    path: &Path,
    items: &[(crate::oracle::trace_types::ValidatedTrace, String)],
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
