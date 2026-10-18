//! Swarm-specific Git preparation, patch checks and artifact inventory.
//!
//! Call [commit::prepare] to commit worker edits with provenance, then
//! [branch::has_net_changes] and [branch::policy::decide] before integration.
//! [commit::inventory] measures files written independently of worker reports.
/// Git branch patch detection and integration decisions.
pub mod branch;
/// Provenance-aware commit preparation and artifact inventory.
pub mod commit;

mod git;

/// Integrate completed subtask worktrees and retain failed artifacts.
pub mod integration;
