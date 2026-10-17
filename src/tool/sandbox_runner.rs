//! Compatibility exports for shared sandbox runner plans.
#[cfg(test)]
use super::SandboxPolicy;
pub(super) use codetether_sandbox_runner::{RunnerPlan, plan};
#[cfg(test)]
#[path = "sandbox_runner_tests.rs"]
mod tests;
