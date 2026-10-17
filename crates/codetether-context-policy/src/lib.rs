//! Relevance extraction and calibrated delegation policy shared across agents.
//!
//! [relevance] classifies message history into contextual buckets. [delegation]
//! tracks per-agent/skill/bucket posteriors and ranks delegation candidates.
/// Posterior updates, risk-aware ranking and configuration.
pub mod delegation;
/// Message relevance signals and context bucket contracts.
pub mod relevance;

/// Derivation strategy contracts and budget policy.
pub mod derivation;

/// Hierarchical summary cache, range contracts and persistence format.
pub mod summary_index;

/// Page classification and minimum-fidelity contracts.
pub mod pages;
/// Source-aware context projection and pairing repair.
pub mod projection;

/// Relevance ranking and prepared-summary selection policy.
pub mod selection;

/// Reusable context transformations and effect statistics.
pub mod transforms;

/// Pinned-message selection and budget reservation.
pub mod pins;
/// Compression-safe pinned-state header formatting.
pub mod state_header;

/// Request assembly and mandatory/optional context-budget policy.
pub mod request;

/// Pure transcript rendering and message text selection.
pub mod transcript;

/// Emergency history and payload compression policy.
pub mod compression;

/// Stable observable faults from context construction and recall.
pub mod fault;
