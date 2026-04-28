//! Hierarchical summary index — Phase B step 14 scaffolding.
//!
//! The Liu et al. paper (arXiv:2512.22087) calls for an incremental
//! per-(range → summary) cache so the prompt loop can re-derive context
//! in `O(log H)` rather than `O(H)` each turn.
//!
//! ## Module layout
//!
//! * [`types`] — [`SummaryRange`], [`Granularity`], [`SummaryNode`]
//! * [`cache`] — [`SummaryIndex`] struct definition + accessors
//! * [`cache_mut`] — mutation methods (insert, append, invalidate, LRU)
//! * [`cache_async`] — async `summary_for` with producer pattern
//! * [`index_produce`](super::index_produce) — RLM-backed production
//!
//! Persisted inline on [`Session`](crate::session::Session) JSON behind
//! `#[serde(default)]` so legacy sessions deserialise unchanged.

pub mod cache;
pub mod cache_async;
pub mod cache_mut;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_async;
#[cfg(test)]
mod tests_summary_for;
pub mod types;

pub use cache::SummaryIndex;
pub use types::{Granularity, SummaryNode, SummaryRange};
