use crate::a2a::spawn::A2APeerHandle;
use crate::{
    bus::{AgentBus, BusHandle},
    cli::A2aArgs,
};
use codetether_a2a_worker_core::initialization::initialize::{Host, Options};
use std::sync::Arc;
pub(super) struct Native;
impl Host for Native {
    type Args = A2aArgs;
    type Client = reqwest::Client;
    type Bus = AgentBus;
    type Peer = A2APeerHandle;
    type Announcement = BusHandle;
    fn apply_access(args: &A2aArgs) {
        let mode = codetether_config::a2a_runtime::auto_approve_access_mode(&args.auto_approve);
        crate::config::Config::apply_process_access_mode_override(mode);
    }
    fn options(args: &A2aArgs) -> Options<'_> {
        Options {
            server: &args.server,
            token: &args.token,
            name: args.name.as_deref(),
            workspaces: args.workspaces.as_deref(),
            max_concurrent_tasks: args.max_concurrent_tasks,
            auto_approve: &args.auto_approve,
        }
    }
    fn client() -> anyhow::Result<reqwest::Client> {
        super::super::build_worker_http_client()
    }
    fn bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
        bus
    }
    fn announce(bus: &Arc<AgentBus>, worker_id: &str) -> BusHandle {
        let handle = bus.handle(worker_id);
        handle.announce_ready(super::super::worker_capabilities());
        super::super::worker_tool_catalog::announce(&handle);
        handle
    }
    async fn peer(args: &A2aArgs, name: &str, bus: Arc<AgentBus>) -> Option<A2APeerHandle> {
        super::super::maybe_start_worker_a2a_peer(args, name, bus).await
    }
}
