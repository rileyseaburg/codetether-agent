//! Verified terminal-goal transitions and stale-result protection.
//!
//! [Host] provides the native verifier and revision-aware storage. [run] creates
//! that verifier; [run_with] accepts an explicit verifier for embedded use.
mod args;
mod commit;
mod host;
mod run;
mod tool;
pub use args::Args;
pub use host::Host;
pub use run::{run, run_with};
pub use tool::UpdateGoalTool;
