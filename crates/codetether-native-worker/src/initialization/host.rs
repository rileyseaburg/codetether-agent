use codetether_a2a_http::peer_handle::A2APeerHandle;
use codetether_a2a_worker_core::initialization::initialize::{Host, Options};
use codetether_bus_core::{AgentBus, BusHandle};
use codetether_cli_options::A2aArgs;
use std::sync::Arc;
pub(super) struct Native<H>(std::marker::PhantomData<fn() -> H>);
impl<H: super::Hooks> Host for Native<H> {
    type Args = A2aArgs;
    type Client = reqwest::Client;
    type Bus = AgentBus;
    type Peer = A2APeerHandle;
    type Announcement = BusHandle;
    fn apply_access(args: &A2aArgs) {
        let mode = codetether_config::a2a_runtime::auto_approve_access_mode(&args.auto_approve);
        codetether_config::Config::apply_process_access_mode_override(mode);
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
        codetether_a2a_http::worker::build_worker_http_client()
    }
    fn bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
        bus
    }
    fn announce(bus: &Arc<AgentBus>, worker_id: &str) -> BusHandle {
        let handle = bus.handle(worker_id);
        handle.announce_ready(codetether_a2a_worker_core::worker_capabilities());
        H::announce(&handle);
        handle
    }
    async fn peer(args: &A2aArgs, name: &str, bus: Arc<AgentBus>) -> Option<A2APeerHandle> {
        H::peer(args, name, bus).await
    }
}
