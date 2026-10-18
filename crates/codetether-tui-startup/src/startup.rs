//! Poll independent startup dependencies together, retaining individual outcomes.
use crate::{Load, Startup};
use codetether_bus_core::AgentBus;
use codetether_config::Config;
use std::{path::Path, sync::Arc};
/// Load providers, worker bridge, session, configuration and workspace concurrently.
/// # Arguments
/// Receives the workspace, local bus and optional explicit session ID.
/// # Returns
/// Independent results without discarding configuration/session failures.
/// # Examples
/// ```no_run
/// async fn load<H: codetether_tui_startup::Load>(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
///     let _startup = codetether_tui_startup::load::<H>(std::path::Path::new("."), bus, None).await;
/// }
/// ```
pub async fn load<H: Load>(
    cwd: &Path,
    bus: Arc<AgentBus>,
    session_id: Option<&str>,
) -> Startup<H::Registry, H::Bridge, H::Tail> {
    let registry_task = H::providers();
    let worker_task = H::worker(bus);
    let session_task = H::session(session_id);
    let config_task = Config::load();
    let workspace_task = crate::workspace::capture(cwd.to_path_buf());
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
