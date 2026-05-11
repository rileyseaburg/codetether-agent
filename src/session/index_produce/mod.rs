//! RLM-backed summary production for [`SummaryIndex::summary_for`].
//!
//! This module is the "producer" half of step 18. The index data
//! structure lives in [`super::index`]; this file owns the async
//! call through the RLM router to materialise a summary.

mod build_context;
mod call;
pub mod summary_text;

pub use call::{SummaryObservability, produce_summary};
