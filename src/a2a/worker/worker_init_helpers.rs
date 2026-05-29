//! Helper functions for worker initialization.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{bus::AgentBus, cli::A2aArgs};

use super::{WorkerTaskRuntime, normalize_max_concurrent_tasks, parse_auto_approve, task_timeline};

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
    args.workspaces
        .as_deref()
        .map(|items| {
            items.split(',').map(|item| item.trim().to_string()).collect()
        })
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()])
}
