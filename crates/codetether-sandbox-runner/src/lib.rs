//! Select a confinement backend and build an owned command execution plan.
//!
//! [selected_runner] resolves the platform backend. [plan] retains seccomp
//! descriptors and Landlock rules until the resulting child is created.
use codetether_sandbox_bwrap as sandbox_bwrap_args;
use codetether_sandbox_bwrap as sandbox_bwrap_probe;
use codetether_sandbox_landlock as sandbox_landlock;
use codetether_sandbox_policy::SandboxPolicy;
use codetether_sandbox_seatbelt as sandbox_seatbelt;
use codetether_sandbox_seccomp as sandbox_seccomp;
mod sandbox_runner;
mod sandbox_runner_bwrap;
mod sandbox_runner_direct;
mod sandbox_runner_seatbelt;
mod sandbox_runner_select;
pub use sandbox_runner::{RunnerPlan, plan};
pub use sandbox_runner_direct::{ENV, enabled};
pub use sandbox_runner_select::{Runner, selected_runner};
#[cfg(test)]
mod plan_tests;
