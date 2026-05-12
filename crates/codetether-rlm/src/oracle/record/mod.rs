//! Serialisable oracle trace record and verdict conversion.
//!
//! [`OracleTraceRecord`] is the on-disk / S3 representation of
//! an oracle result.  The [`OracleResult::to_record`] method in
//! the sibling [`convert`] module flattens the enum into this
//! flat struct for JSON serialisation.
//!
//! # Examples
//!
//! ```ignore
//! let record = oracle_result.to_record();
//! assert_eq!(record.verdict, "golden");
//! ```

mod convert;

use serde::{Deserialize, Serialize};

use super::trace_types::ValidatedTrace;

/// Flat, serialisable snapshot of an oracle verdict plus its
/// full trace — the unit of persistence for the spool and S3.
///
/// Produced by [`super::trace_types::OracleResult::to_record`].
///
/// # Examples
///
/// ```ignore
/// let record = OracleTraceRecord {
///     verdict: "golden".into(),
///     reason: None,
///     agreement_ratio: None,
///     trace: validated_trace,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleTraceRecord {
    /// Classification string: `"golden"`, `"consensus"`,
    /// `"unverified"`, or `"failed"`.
    pub verdict: String,
    /// Human-readable explanation when the verdict is not golden.
    pub reason: Option<String>,
    /// Consensus agreement ratio, present only for the
    /// `Consensus` variant.
    pub agreement_ratio: Option<f32>,
    /// Complete validated trace snapshot.
    pub trace: ValidatedTrace,
}
