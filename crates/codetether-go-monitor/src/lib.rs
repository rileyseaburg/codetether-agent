//! Background autonomous-pipeline monitoring.
//!
//! Launchers register an [`ActiveGoRun`] in [`ACTIVE_GO_RUNS`] and update its
//! [`GoRunPhase`]. Call [`watch`] to list runs or read one run, its PRD story
//! progress, and its last thirty progress-note lines.

mod active;
mod header;
mod list;
mod phase;
mod phase_display;
mod progress;
mod stories;
mod watch;
pub use active::{ACTIVE_GO_RUNS, ActiveGoRun};
pub use phase::GoRunPhase;
pub use watch::watch;
