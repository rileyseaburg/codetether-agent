//! RLM-backed summary production for [`SummaryIndex::summary_for`].
//!
//! This module is the "producer" half of step 18. The index data
//! structure lives in [`super::index`]; this file owns the async
//! call through the RLM router to materialise a summary.

pub(crate) mod notify;
mod observability;
#[path = "../proactive_rlm/mod.rs"]
pub(crate) mod proactive;
pub use codetether_summary_quality as summary_gate;
pub use codetether_summary_quality as summary_text;

pub use codetether_native_proactive::summary::produce_summary;
pub use observability::SummaryObservability;
