//! Worker swarm adapters: session preparation, controlled execution, and handoff.

mod access;
mod context;
mod entry;
mod execute;
mod finish;
mod handoff;
mod prepare;
mod swarm_policy_run;
use codetether_swarm_runtime::inline as swarm_policy_start;

pub(super) use entry::execute_swarm_with_policy;
