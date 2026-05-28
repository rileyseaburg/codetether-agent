//! Crate-level and worker-core re-export glue for the worker module tree.

pub(super) use crate::{
    a2a::{
        git_credentials::{configure_repo_git_auth, write_git_credential_helper_script},
        task_scope::check_task_scope,
        worker_tool_registry::{create_filtered_registry, is_tool_allowed},
        worker_workspace_context::resolve_task_workspace_dir,
        worker_workspace_record::{RegisteredWorkspaceRecord, fetch_workspace_record},
    },
    provenance::install_commit_msg_hook,
    session::helper::runtime::enrich_tool_input_with_runtime_context,
};
pub(super) use codetether_a2a_worker_core::{
    CognitionLatestSnapshot, CognitionStatusSnapshot, export_worker_runtime_env, is_forage_agent,
    is_swarm_agent, model_ref_to_provider_model, normalize_max_concurrent_tasks, resolve_worker_id,
    worker_capabilities,
};
