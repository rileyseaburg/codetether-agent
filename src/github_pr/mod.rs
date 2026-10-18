//! Native compatibility facade for shared PR services and CLI execution.
pub mod autopilot;
pub mod ci_watcher;
pub mod provenance_block;

pub use codetether_github_pr_cli::run;
