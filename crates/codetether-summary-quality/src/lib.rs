//! Quality checks and output limits for RLM-produced summaries.
//!
//! [bounded_summary] rejects failed or empty results. [try_bounded_summary]
//! also rejects implausibly small output for a large input. [clamp_tokens]
//! bounds accepted text with the existing UTF-8-safe truncation marker.
mod bounded;
mod clamp;
mod floor;
mod gate;
pub use bounded::bounded_summary;
pub use clamp::clamp_tokens;
pub use codetether_context_policy::summary_index::text::strip_stats_header;
pub use gate::try_bounded_summary;
#[cfg(test)]
mod tests;
