use std::path::Path;
use std::sync::Arc;

use crate::bus::AgentBus;
use crate::config::Config;
use crate::provider::ProviderRegistry;
use crate::session::TailLoad;
use crate::tui::models::WorkspaceSnapshot;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) struct Startup {
    pub registry: Option<Arc<ProviderRegistry>>,
    pub worker_bridge: Option<TuiWorkerBridge>,
    pub session_load: Option<anyhow::Result<TailLoad>>,
    pub config: Option<anyhow::Result<Config>>,
    pub workspace: WorkspaceSnapshot,
}

pub(super) async fn load(cwd: &Path, bus: Arc<AgentBus>) -> Startup {
    let registry_task = super::provider::load_registry();
    let worker_task =
        TuiWorkerBridge::spawn(None, None, crate::provenance::runtime_agent_identity(), bus);
    let session_task = super::session_scan::load(cwd);
    let config_task = Config::load();
    let workspace_task = super::workspace::capture(cwd.to_path_buf());
    let (registry, worker_result, session_load, config, workspace) = tokio::join!(
        registry_task,
        worker_task,
        session_task,
        config_task,
        workspace_task
    );
    Startup {
        registry,
        worker_bridge: worker_result.ok().flatten(),
        session_load: Some(session_load),
        config: Some(config),
        workspace,
    }
}
