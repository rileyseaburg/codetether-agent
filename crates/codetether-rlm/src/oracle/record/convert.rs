//! Conversion from [`OracleResult`] enum to the flat
//! [`OracleTraceRecord`] struct used for persistence.
//!
//! Each enum variant is mapped to a `verdict` string with
//! optional `reason` and `agreement_ratio` metadata.
//!
//! # Examples
//!
//! ```ignore
//! let record = OracleResult::Golden(trace).to_record();
//! assert_eq!(record.verdict, "golden");
//! ```

use super::super::trace_types::OracleResult;
use super::OracleTraceRecord;

impl OracleResult {
    /// Flatten the verdict enum into a serialisable record.
    ///
    /// Copies the inner [`super::super::trace_types::ValidatedTrace`]
    /// and maps the variant to a `verdict` string suitable for
    /// JSON export and S3 key partitioning.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let record = result.to_record();
    /// serde_json::to_string(&record).unwrap();
    /// ```
    pub fn to_record(&self) -> OracleTraceRecord {
        match self {
            Self::Golden(trace) => OracleTraceRecord {
                verdict: "golden".into(),
                reason: None,
                agreement_ratio: None,
                trace: trace.clone(),
            },
            Self::Consensus {
                trace,
                agreement_ratio,
            } => OracleTraceRecord {
                verdict: "consensus".into(),
                reason: None,
                agreement_ratio: Some(*agreement_ratio),
                trace: trace.clone(),
            },
            Self::Unverified { reason, trace } => OracleTraceRecord {
                verdict: "unverified".into(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            },
            Self::Failed { reason, trace, .. } => OracleTraceRecord {
                verdict: "failed".into(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            },
        }
    }
}
