//! Active-task anchoring, relevance scores and prepared-summary packing.
//!
//! [active_tail] protects the substantive user task. [scoring] ranks older
//! messages; [pack] selects prepared summaries before optional raw history.
use crate::relevance::{RelevanceMeta, extract};
use codetether_provider_core::Message;
/// Active-task boundaries and continuation recognition.
pub mod active_tail;
/// Prepared-summary and raw-history budget packing.
pub mod pack;
/// Cached summary range selection.
pub mod prepared_gaps;
/// Omitted transcript ranges.
pub mod ranges;
/// Message scores and current-task signatures.
pub mod scoring;
pub use scoring::{DEFAULT_RECENT_WINDOW, score_messages, task_signature};
#[cfg(test)]
mod tests;
