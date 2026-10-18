//! A2A worker implementation split into SRP-focused submodules.

#[rustfmt::skip]
mod a2a_peer;
mod auto_approve;
#[cfg(test)]
mod extended_heartbeat;
use codetether_a2a_http::worker as heartbeat_loop;
mod pending_tasks;
mod public_api;
mod reexports;
mod reexports_a;
mod reexports_b;
mod reexports_c;
mod reexports_crate;
mod run;
mod run_with_state;
mod session_policy;
mod session_steps;
mod swarm_policy;
#[rustfmt::skip]
mod task_dispatch_special;
mod task_execute;
mod task_execute_title;
#[cfg(test)]
mod task_execute_title_tests;
mod task_session;
#[rustfmt::skip]
#[cfg(test)] mod test_forage;
#[cfg(test)]
mod test_public;
#[cfg(test)]
mod test_runtime;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod test_support_provider;
#[cfg(test)]
mod test_tool_schema_retry;
mod worker_init;
mod worker_security;

pub use public_api::*;
use reexports::*;

#[cfg(test)]
include!("mod_test_imports.rs");
