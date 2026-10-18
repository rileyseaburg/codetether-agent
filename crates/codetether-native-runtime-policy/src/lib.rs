//! Application runtime-policy evaluation and sandbox preflight.
//!
//! [evaluate_tool_invocation] loads effective configuration and returns a
//! structured rejection when an invocation is denied. [RuntimeToolPolicy]
//! represents reusable decisions; evaluation uses shared prior-context and
//! sandbox-runtime probes without depending on the application crate.
use codetether_config as config;
#[cfg(test)]
use codetether_native_approval as approval;
#[cfg(test)]
use codetether_provider_core as provider;
use codetether_tool_core as tool;
mod runtime_policy;
pub use runtime_policy::*;
