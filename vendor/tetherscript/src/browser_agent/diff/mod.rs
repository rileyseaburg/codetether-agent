//! Deterministic DOM diff summaries for agent traces.
//!
//! This module reports coarse path-level insertions, removals, text changes,
//! and attribute changes. It intentionally avoids edit-distance matching so
//! trace output stays deterministic and cheap.

mod attrs;
mod compare;
mod entry;
mod path;
mod summary;
mod types;
pub mod visual;
mod walk;

pub use summary::{diff_documents, diff_html};
pub use types::{DomDiffEntry, DomDiffKind, DomDiffSummary};

#[cfg(test)]
mod tests;
