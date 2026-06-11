//! A2A worker implementation split into SRP-focused submodules.

#[rustfmt::skip]
mod a2a_peer;mod a2a_peer_config;
mod auto_approve;
mod clone_enqueue;
mod clone_git;
mod clone_git_cmd;
mod clone_location;
mod clone_lock;
mod clone_lock_release;
mod clone_lock_stale;
mod clone_register;
mod clone_repo_exec;
mod clone_target;
mod clone_task;
mod clone_task_data;
mod clone_task_result;
mod completion_retry;
mod extended_heartbeat;
mod forage_args;
mod forage_settings;
mod forage_task;
mod git_branch;
#[cfg(test)]
mod git_branch_tests;
#[rustfmt::skip]
mod git_commit_push;mod git_commit_push_ops;
mod git_commit_push_provenance;
mod git_refspec;
mod heartbeat_cognition;
mod heartbeat_cognition_merge;
mod heartbeat_loop;
mod http_client;
mod metadata_flags;
mod metadata_lookup;
mod metadata_numbers;
mod metadata_strings;
mod model_defaults;
mod model_preferences;
mod pending_tasks;
mod persistent_heartbeat;
mod post_clone_task;
mod provider_models;
mod public_api;
mod reexports;
mod reexports_a;
mod reexports_b;
mod reexports_c;
mod reexports_crate;
mod registration_data;
mod release;
mod release_payload;
mod run;
mod run_with_state;
mod runtime_state;
mod server_state;
mod session_policy;
mod session_registry;
mod session_steps;
mod swarm_event_output;
mod swarm_model;
mod swarm_policy;
mod swarm_policy_result;
mod swarm_setup;
mod task_agent;
mod task_agent_swarm_result;
mod task_claim;
mod task_context;
mod task_context_helpers;
mod task_context_struct;
mod task_data;
#[rustfmt::skip] mod task_dispatch;mod task_dispatch_special;
mod task_execute;
mod task_finalize;
mod task_handler;
mod task_handler_release;
mod task_outcome;
mod task_output;
mod task_policy_args;
mod task_release;
mod task_session;
mod task_session_resolve;
mod task_slot;
mod task_stream;
mod task_stream_request;
pub(crate) mod task_timeline;
mod task_timeout;
#[cfg(test)]
mod test_forage;
#[cfg(test)]
mod test_post_clone;
#[cfg(test)]
mod test_public;
#[cfg(test)]
mod test_runtime;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod test_support_provider;
#[cfg(test)]
mod test_tool_schema;
#[cfg(test)]
mod test_tool_schema_retry;
mod tool_schema;
mod worker_bootstrap;
mod worker_context;
mod worker_env;
mod worker_init;
mod worker_init_helpers;
mod worker_init_runtime;
mod worker_loop;
mod worker_registration;
mod worker_server_loop;
pub(super) mod workspace_resolve;
mod workspace_scope;
mod workspace_sync;
mod workspace_sync_git;
mod workspace_sync_start;

pub use public_api::*;
use reexports::*;
