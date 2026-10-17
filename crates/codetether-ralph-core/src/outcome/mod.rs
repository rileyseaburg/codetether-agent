//! Shared interpretation of story and conflict-resolution outcomes.
//!
//! [conflict_resolved] classifies resolver text; [story_failure] retains a
//! supplied failure or selects the established fallback. Hosts report results.
mod conflict;
mod failure;
pub use conflict::conflict_resolved;
pub use failure::story_failure;
