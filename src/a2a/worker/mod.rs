//! A2A worker implementation split into SRP-focused submodules.

#[rustfmt::skip]
mod a2a_peer;
mod auto_approve;
use codetether_a2a_worker_clone as clone_task;
use codetether_worktree::repository as clone_target;
mod completion_retry;
mod extended_heartbeat;
mod forage_task;
#[cfg(test)]
mod git_branch_tests;
#[rustfmt::skip]
mod connection_setup;
mod git_hook_gate;
use codetether_a2a_http::worker as heartbeat_loop;
use codetether_a2a_http::worker as http_client;
use codetether_a2a_worker_core::task_config as metadata_flags;
use codetether_a2a_worker_core::task_config as metadata_lookup;
use codetether_a2a_worker_core::task_config as metadata_numbers;
use codetether_a2a_worker_core::task_config as metadata_strings;
use codetether_worktree::repository as git_refspec;
mod model_defaults;
use codetether_a2a_worker_core::execution_policy as model_preferences;
mod pending_tasks;
mod provider_models;
mod public_api;
mod reconnect_host;
mod reexports;
mod reexports_a;
mod reexports_b;
mod reexports_c;
mod reexports_crate;
use codetether_a2a_worker_core::registration as registration_data;
mod run;
mod run_with_state;
mod runtime_state;
mod server_state;
use codetether_a2a_worker_core::events as session_event;
use codetether_a2a_worker_core::events::sink as session_event_sink;
mod session_policy;
mod session_registry;
use codetether_a2a_worker_core::task_config as session_step_budget;
mod session_steps;
use codetether_swarm_core::worker_output as swarm_event_output;
mod swarm_model;
mod swarm_policy;
use codetether_swarm_core::worker_result as swarm_policy_result;
mod swarm_setup;
mod task_agent;
use codetether_a2a_worker_core::execution_policy as task_agent_swarm_result;
mod task_claim;
use codetether_a2a_worker_core::task_config as task_context;
use codetether_a2a_worker_core::task_config as task_context_helpers;
use codetether_a2a_worker_core::task_config as task_data;
mod task_dispatch;
#[rustfmt::skip]
mod task_dispatch_special;
mod task_execute;
#[rustfmt::skip]
mod task_execute_sink;mod task_execute_title;
#[cfg(test)]
mod task_execute_title_tests;
mod task_handler;
#[rustfmt::skip]
mod task_handler_release;use codetether_a2a_http::worker::output_callback as task_output;
use codetether_a2a_worker_core::dispatch as task_handler_timeout;
use codetether_a2a_worker_core::outcome as task_outcome;
use codetether_a2a_worker_core::task_config::policy_args as task_policy_args;
mod task_session;
mod task_session_resolve;
use codetether_a2a_worker_core::dispatch as task_slot;
mod task_stream;
use codetether_a2a_http::worker as task_stream_error;
#[rustfmt::skip]
mod task_stream_request;use codetether_a2a_worker_core::dispatch as task_targeting;
pub(crate) mod task_timeline;
use codetether_a2a_worker_core::task_config as task_timeout;
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
mod worker_bootstrap;
mod worker_context;
use codetether_a2a_worker_core as worker_env;
mod worker_init;
mod worker_loop;
mod worker_security;
mod worker_server_loop;
mod worker_tool_catalog;
mod workspace_scope;
mod workspace_sync;
use codetether_worktree::repository::credentials as workspace_sync_git;
mod workspace_sync_start;

pub use public_api::*;
use reexports::*;
