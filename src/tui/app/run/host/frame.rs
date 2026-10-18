//! Existing frame drawing and session hydration callbacks.
use super::*;
impl codetether_tui_startup::Frame for Native {
    type App = App;
    type Session = Session;
    fn draw(terminal: &mut Terminal, cwd: &Path, network: bool) -> anyhow::Result<App> {
        codetether_tui_startup::first_frame::draw::<Native>(terminal, cwd, network)
    }
    async fn resolve(
        scan: Option<anyhow::Result<TailLoad>>,
        bus: &Arc<AgentBus>,
    ) -> anyhow::Result<Resolved> {
        session_resolve::resolve(scan, bus).await
    }
    fn initial(app: &mut App, cwd: &Path, network: bool, peer: bool, session: &Session) {
        hydrate_initial::initial(app, cwd, network, peer, session);
    }
    fn complete(
        app: &mut App,
        network: bool,
        session: &mut Session,
        bridge: Option<&TuiWorkerBridge>,
        outcome: SessionLoadOutcome,
        workspace: WorkspaceSnapshot,
    ) {
        hydrate::complete(app, network, session, bridge, outcome, workspace);
    }
}
