//! Helper functions for worker initialization.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{bus::AgentBus, cli::A2aArgs, config::Config};

use super::{WorkerTaskRuntime, normalize_max_concurrent_tasks, parse_auto_approve, task_timeline};

/// Align the process-wide runtime access mode with the worker's `--auto-approve`
/// policy.
///
/// The worker's own tool registry honours `--auto-approve`, but runtime policy
/// (which gates `bash`, patches, and other side-effecting tools) reads the
/// access mode from config. Interactive entrypoints set that override from their
/// CLI flags; the worker did not, so `--auto-approve all` still produced
/// "requires approval" tool failures with no human present to approve them,
/// failing every task that needed to clone or modify a repository.
pub(super) fn apply_auto_approve_access_mode(args: &A2aArgs) {
    let access_mode = auto_approve_access_mode(&args.auto_approve);
    Config::apply_process_access_mode_override(access_mode);
}
pub(super) use codetether_config::a2a_runtime::auto_approve_access_mode;

pub(super) fn build_task_runtime(
    args: &A2aArgs,
    client: &reqwest::Client,
    server: &str,
    worker_id: &str,
    name: &str,
    processing: &super::worker_init_runtime::ProcessingSet,
    bus: &Arc<AgentBus>,
    progress: &Arc<Mutex<task_timeline::TaskProgressState>>,
) -> WorkerTaskRuntime {
    WorkerTaskRuntime {
        client: client.clone(),
        server: server.to_string(),
        token: args.token.clone(),
        worker_id: worker_id.to_string(),
        agent_name: name.to_string(),
        processing: processing.clone(),
        max_concurrent_tasks: normalize_max_concurrent_tasks(args.max_concurrent_tasks),
        auto_approve: parse_auto_approve(&args.auto_approve),
        bus: bus.clone(),
        task_progress: progress.clone(),
        workspace_ids: Vec::new(),
    }
}
pub(super) fn parse_codebases(args: &A2aArgs) -> Vec<String> {
    codetether_a2a_worker_core::initialization::parse_codebases(args.workspaces.as_deref())
}
pub(super) use codetether_a2a_worker_core::initialization::resolve_name;
