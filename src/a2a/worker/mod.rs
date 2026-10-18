//! A2A worker implementation split into SRP-focused submodules.

#[rustfmt::skip]
mod a2a_peer;
mod auto_approve;
use codetether_a2a_worker_clone as clone_task;
use codetether_worktree::repository as clone_target;
use codetether_native_worker::completion as completion_retry;
mod extended_heartbeat;
use codetether_a2a_http::worker as heartbeat_loop;
use codetether_a2a_http::worker as http_client;
use codetether_a2a_worker_core::task_config as metadata_flags;
use codetether_a2a_worker_core::task_config as metadata_lookup;
use codetether_a2a_worker_core::task_config as metadata_numbers;
use codetether_a2a_worker_core::task_config as metadata_strings;
use codetether_worktree::repository as git_refspec;
use codetether_native_worker::model_policy as model_defaults;
use codetether_a2a_worker_core::execution_policy as model_preferences;
mod pending_tasks;
use codetether_native_providers::worker_models as provider_models;
mod public_api;
mod reexports;
mod reexports_a;
mod reexports_b;
mod reexports_c;
mod reexports_crate;
use codetether_a2a_worker_core::registration as registration_data;
mod run;
mod run_with_state;
use codetether_native_worker::runtime_state;
use codetether_a2a_worker_core::events as session_event;
use codetether_a2a_worker_core::events::sink as session_event_sink;
mod session_policy;
use codetether_a2a_worker_core::task_config as session_step_budget;
mod session_steps;
use codetether_swarm_core::worker_output as swarm_event_output;
mod swarm_policy;
use codetether_swarm_core::worker_result as swarm_policy_result;
use codetether_a2a_worker_core::execution_policy as task_agent_swarm_result;
use codetether_a2a_worker_core::task_config as task_context;
use codetether_a2a_worker_core::task_config as task_context_helpers;
use codetether_a2a_worker_core::task_config as task_data;
#[rustfmt::skip]
mod task_dispatch_special;
mod task_execute;
mod task_execute_title;
#[cfg(test)]
mod task_execute_title_tests;
use codetether_a2a_http::worker::output_callback as task_output;
use codetether_a2a_worker_core::dispatch as task_handler_timeout;
use codetether_a2a_worker_core::outcome as task_outcome;
use codetether_a2a_worker_core::task_config::policy_args as task_policy_args;
mod task_session;
use codetether_a2a_worker_core::dispatch as task_slot;
use codetether_a2a_http::worker as task_stream_error;
use codetether_a2a_worker_core::dispatch as task_targeting;
use codetether_a2a_worker_core::task_config as task_timeout;
pub(crate) use codetether_a2a_worker_core::task_timeline;
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
use codetether_native_worker::worker_context;
use codetether_a2a_worker_core as worker_env;
mod worker_init;
mod worker_security;
use codetether_native_worker::workspace_scope;
use codetether_native_worker::workspace_sync;
use codetether_worktree::repository::credentials as workspace_sync_git;
use codetether_native_worker::workspace_sync_start;

pub use public_api::*;
use reexports::*;
