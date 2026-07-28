//! Helper functions for worker initialization.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    bus::AgentBus,
    cli::A2aArgs,
    config::{AccessMode, Config},
};

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

/// Map a worker auto-approve policy onto the equivalent runtime access mode.
///
/// Only the unattended `all` policy widens the access mode; `safe` and `none`
/// leave the configured value untouched so their prompts still apply.
pub(super) fn auto_approve_access_mode(auto_approve: &str) -> Option<AccessMode> {
    match auto_approve.trim() {
        "all" => Some(AccessMode::Full),
        _ => None,
    }
}

#[cfg(test)]
mod auto_approve_access_mode_tests {
    use super::auto_approve_access_mode;
    use crate::config::AccessMode;

    #[test]
    fn unattended_all_policy_grants_full_access() {
        assert!(matches!(
            auto_approve_access_mode("all"),
            Some(AccessMode::Full)
        ));
        assert!(matches!(
            auto_approve_access_mode(" all "),
            Some(AccessMode::Full)
        ));
    }

    #[test]
    fn narrower_policies_keep_configured_access_mode() {
        assert!(auto_approve_access_mode("safe").is_none());
        assert!(auto_approve_access_mode("none").is_none());
        assert!(auto_approve_access_mode("").is_none());
    }
}

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
            items
                .split(',')
                .map(|item| item.trim().to_string())
                .collect()
        })
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()])
}

pub(super) fn resolve_name(configured: Option<&str>) -> String {
    match configured {
        Some(name) => crate::provenance::bind_runtime_agent_identity(name),
        None => crate::provenance::ensure_runtime_agent_identity(&format!(
            "codetether-{}",
            std::process::id()
        )),
    }
}
