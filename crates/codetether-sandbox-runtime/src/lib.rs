//! Sandboxed child spawning, bounded execution and confinement reporting.
//!
//! [spawn] leaves a child running; [execute_sandboxed] waits for completion.
//! Policy validation and runner planning precede both paths. Retained plans
//! keep filter resources alive while child processes are created.
pub use codetether_plugin_verification::{
    PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file,
};
use codetether_sandbox_environment as sandbox_env;
use codetether_sandbox_landlock as sandbox_landlock;
pub use codetether_sandbox_policy::{SandboxPolicy, SandboxResult};
use codetether_sandbox_runner as sandbox_runner;
use codetether_sandbox_runner as sandbox_runner_select;
use codetether_sandbox_runner as sandbox_runner_direct;
use codetether_sandbox_seccomp as sandbox_seccomp;
mod sandbox_availability;
mod sandbox_command;
mod sandbox_execute;
mod sandbox_limits;
mod sandbox_plan_state;
mod sandbox_process;
mod sandbox_result_builder;
mod sandbox_spawn;
pub use sandbox_availability::{direct_fallback_env_allowed, unavailable_reason};
pub use sandbox_execute::execute_sandboxed;
pub use sandbox_spawn::{Spawned, spawn};
#[cfg(test)]
mod policy_tests;
#[cfg(all(test, unix))]
mod toolchain_tests;
