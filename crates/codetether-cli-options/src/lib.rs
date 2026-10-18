//! Shared CLI option schemas, independent of command execution.
//!
//! These Parser and Subcommand types retain the application CLI defaults,
//! aliases, help text and environment bindings. Native CLI paths re-export them.
mod a2a_args;
mod benchmark_args;
mod create_pr_args;
mod exports;
mod forage_args;
mod git_credential_helper_args;
mod mcp_args;
mod moltbook_args;
mod moltbook_command;
mod moltbook_comment_args;
mod moltbook_post_args;
mod moltbook_register_args;
mod moltbook_search_args;
mod moltbook_update_profile_args;
mod okr_args;
mod pr_args;
mod pr_command;
mod ralph_args;
mod rlm_args;
mod serve_args;
mod spawn_args;
mod stats_args;
mod swarm_args;
mod worker_server_args;
pub use exports::*;
