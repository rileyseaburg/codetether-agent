//! Legacy executor decoding and trace-evidence policies.
//!
//! Helpers decode model text and sub-query expressions, inspect [TraceStep]
//! records, construct canonical grep payloads, and bound diagnostic text.
//! The caller owns provider requests, loop state, and trace mutation.
use crate::oracle::{FinalPayload, GrepMatch, GrepOracle, GrepPayload, QueryType, TraceStep};
use crate::repl::RlmRepl;
mod coerce_grep_payload_from_trace;
pub use coerce_grep_payload_from_trace::coerce_grep_payload_from_trace;
mod has_pattern_evidence;
pub use has_pattern_evidence::has_pattern_evidence;
mod extract_latest_grep_matches;
pub use extract_latest_grep_matches::extract_latest_grep_matches;
mod parse_line_numbered_output;
pub use parse_line_numbered_output::parse_line_numbered_output;
mod infer_file_from_query;
pub use infer_file_from_query::infer_file_from_query;
mod trace_output_for_storage;
pub use trace_output_for_storage::trace_output_for_storage;
mod extract_code;
pub use extract_code::extract_code;
mod parse_llm_query;
pub use parse_llm_query::parse_llm_query;
mod truncate_with_ellipsis;
pub use truncate_with_ellipsis::truncate_with_ellipsis;
#[cfg(test)]
mod tests;

/// Legacy prompts and sub-query request construction.
pub mod prompts;

/// Final-payload normalization and trace recording.
pub mod final_payload;

/// Provider execution and response collection for a legacy sub-query.
pub mod subquery;
