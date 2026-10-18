//! Native peer lifetime, access policy and event-loop bindings.
use super::*;
use crate::{a2a::spawn::SpawnOptions, bus::recorder::BusRecorder};
impl codetether_tui_startup::Runtime for Native {
    type Peer = peer::PeerEndpoint;
    type Channels = channels::SessionChannels;
    async fn peer(options: Option<SpawnOptions>, bus: Arc<AgentBus>) -> peer::PeerEndpoint {
        peer::start(
            options,
            bus,
            crate::a2a::spawn::start_a2a_in_background,
            crate::a2a::live_inbox::attach,
        )
        .await
    }
    fn peer_ready(peer: &peer::PeerEndpoint) -> bool {
        peer.ready
    }
    fn policies(
        session: &mut Session,
        startup: &mut Startup,
        access: Option<AccessMode>,
        yolo: bool,
    ) {
        config::apply_policies(session, startup, access, yolo);
    }
    fn channels() -> channels::SessionChannels {
        channels::session()
    }
    async fn run_loop(
        terminal: &mut Terminal,
        app: &mut App,
        cwd: &Path,
        registry: Option<Arc<ProviderRegistry>>,
        session: Session,
        recorder: Arc<BusRecorder>,
        bridge: Option<TuiWorkerBridge>,
        channels: channels::SessionChannels,
    ) -> anyhow::Result<()> {
        loop_run::run(
            terminal, app, cwd, registry, session, recorder, bridge, channels,
        )
        .await
    }
}
