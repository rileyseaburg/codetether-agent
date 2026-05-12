//! Primitive types shared across oracle submodules.
//!
//! Provides [`TraceStep`] for recording individual tool
//! invocations and [`VerificationMethod`] for tagging which
//! oracle backend produced a verdict.
//!
//! # Examples
//!
//! ```ignore
//! use codetether_agent::rlm::oracle::TraceStep;
//! let step = TraceStep { iteration: 1, action: "grep(\"TODO\")".into(), output: "3 matches".into() };
//! ```

use serde::{Deserialize, Serialize};

/// A single tool invocation recorded during an RLM REPL session.
///
/// Each step captures the iteration counter, a human-readable
/// description of the action, and the tool's output.  The
/// resulting `Vec<TraceStep>` is embedded in [`super::trace_types::ValidatedTrace`]
/// for downstream golden-trace export.
///
/// # Examples
///
/// ```ignore
/// let step = TraceStep {
///     iteration: 2,
///     action: "rlm_grep(\"async fn\")".into(),
///     output: "L12: pub async fn run()".into(),
/// };
/// assert_eq!(step.iteration, 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    /// 1-indexed loop counter within the REPL session.
    pub iteration: usize,
    /// Human-readable action description, e.g. `"grep(\"async fn\")"`.
    pub action: String,
    /// Raw or truncated tool output captured for the trace.
    pub output: String,
}

/// Identifies which oracle backend produced a verdict.
///
/// Attached to every [`super::trace_types::ValidatedTrace`] so
/// consumers know the provenance of the classification.
///
/// # Examples
///
/// ```ignore
/// let method = VerificationMethod::GrepOracle;
/// assert_eq!(method, VerificationMethod::GrepOracle);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationMethod {
    /// No oracle was able to verify the trace.
    None,
    /// Verified by line-oriented grep pattern matching.
    GrepOracle,
    /// Verified by tree-sitter AST structural checks.
    AstOracle,
    /// Verified by multi-run agreement consensus.
    Consensus,
}
