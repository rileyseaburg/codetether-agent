//! Workspace clone task orchestration for A2A workers.
//!
//! [handle_clone_repo_task] resolves a registered workspace, installs a
//! temporary credential helper, acquires the workspace lock, clones or
//! refreshes the checkout, registers it, and enqueues an optional follow-up.
//! Git primitives belong to worktree; control-plane primitives belong to HTTP.
use anyhow::{Context, Result};
use codetether_a2a_http::workspace::{
    fetch_workspace_record, git_clone_base_dir, register_cloned_workspace,
    resolve_workspace_clone_path,
};
use codetether_a2a_worker_core::env_u64;
use codetether_a2a_worker_core::task_config::{metadata_str, task_str};
use codetether_provenance::install_commit_msg_hook;
use codetether_worktree::repository::credentials::{
    configure_repo_git_auth, configure_repo_git_github_app_from_agent_config,
    write_git_credential_helper_script,
};
use codetether_worktree::repository::git::refresh_existing_clone;
use post_clone_task::worker_should_enqueue_post_clone_task;
use reqwest::Client;
mod clone_enqueue;
mod clone_lock;
mod clone_lock_release;
mod clone_repo_exec;
mod clone_task;
mod clone_task_data;
mod clone_task_result;
mod enqueue_response;
mod post_clone_task;
pub use clone_task::handle_clone_repo_task;
#[cfg(test)]
mod test_post_clone;

use clone_enqueue::enqueue_post_clone_task;
use clone_lock::{acquire_workspace_clone_lock, workspace_clone_lock_path};
use clone_lock_release::release_workspace_clone_lock;
use clone_task_data::CloneRepoTask;
use clone_task_result::handle_clone_repo_task_result;
