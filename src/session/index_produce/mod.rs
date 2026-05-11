//! RLM-backed summary production for [`SummaryIndex::summary_for`].
//!
//! This module is the "producer" half of step 18. The index data
//! structure lives in [`super::index`]; this file owns the async
//! call through the RLM router to materialise a summary.

mod build_context;
mod call;
mod observability;
pub mod summary_gate;
pub mod summary_text;

#[cfg(test)]
mod summary_gate_tests;
#[cfg(test)]
mod summary_text_tests;

pub use call::produce_summary;
pub use observability::SummaryObservability;
