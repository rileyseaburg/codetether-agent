//! Native startup operations grouped by responsibility.
use crate::{Resolved, Startup};
use codetether_a2a_http::peer_startup::SpawnOptions;
use codetether_bus_core::{AgentBus, recorder::BusRecorder};
use codetether_config::AccessMode;
use codetether_session_resume::SessionLoadOutcome;
use codetether_workspace_snapshot::WorkspaceSnapshot;
use std::{future::Future, path::Path, sync::Arc};
/// Concrete terminal used by the existing shared terminal runtime.
pub type Terminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;
/// Independent native dependency loaders.
/// # Implementors
/// Preserve native provider constructors, worker identity and session loading.
pub trait Load {
    /// Native provider registry.
    type Registry;
    /// Native external-worker bridge.
    type Bridge;
    /// Loaded native session tail.
    type Tail;
    /// Load providers using the native startup policy.
    fn providers() -> impl Future<Output = Option<Arc<Self::Registry>>>;
    /// Start the native worker bridge.
    /// # Errors
    /// Returns bridge startup errors, which loading treats as unavailable.
    fn worker(bus: Arc<AgentBus>) -> impl Future<Output = anyhow::Result<Option<Self::Bridge>>>;
    /// Load a selected session or report the fresh-session reason.
    /// # Errors
    /// Returns session-loading failures for the resolver to classify.
    fn session(id: Option<&str>) -> impl Future<Output = anyhow::Result<Self::Tail>>;
}
/// Native frame rendering and session hydration.
/// # Implementors
/// Keep UI/session field changes in the existing native helpers.
pub trait Frame: Load {
    /// Native application state.
    type App;
    /// Native session.
    type Session;
    /// Render the immediate startup frame.
    /// # Errors
    /// Returns terminal rendering errors.
    fn draw(terminal: &mut Terminal, cwd: &Path, network: bool) -> anyhow::Result<Self::App>;
    /// Resolve session-load outcome.
    /// # Errors
    /// Returns native session creation or loading errors.
    fn resolve(
        scan: Option<anyhow::Result<Self::Tail>>,
        bus: &Arc<AgentBus>,
    ) -> impl Future<Output = anyhow::Result<Resolved<Self::Session>>>;
    /// Hydrate fields known before configuration is applied.
    fn initial(app: &mut Self::App, cwd: &Path, network: bool, peer: bool, session: &Self::Session);
    /// Apply final session/workspace state to the UI.
    fn complete(
        app: &mut Self::App,
        network: bool,
        session: &mut Self::Session,
        bridge: Option<&Self::Bridge>,
        outcome: SessionLoadOutcome,
        workspace: WorkspaceSnapshot,
    );
}
/// Peer, policy and event-loop startup operations.
/// # Implementors
/// Retain peer guards and delegate execution to the existing native event loop.
pub trait Runtime: Frame {
    /// Guard retaining the native peer endpoint.
    type Peer;
    /// Native session event channels.
    type Channels;
    /// Start the optional peer endpoint.
    fn peer(options: Option<SpawnOptions>, bus: Arc<AgentBus>) -> impl Future<Output = Self::Peer>;
    /// Whether startup obtained a usable peer endpoint.
    fn peer_ready(peer: &Self::Peer) -> bool;
    /// Apply configuration and user access policy in the existing order.
    fn policies(
        session: &mut Self::Session,
        startup: &mut Startup<Self::Registry, Self::Bridge, Self::Tail>,
        access: Option<AccessMode>,
        yolo: bool,
    );
    /// Allocate the native session channels.
    fn channels() -> Self::Channels;
    /// Execute the native TUI event loop.
    /// # Errors
    /// Returns native loop errors.
    fn run_loop(
        terminal: &mut Terminal,
        app: &mut Self::App,
        cwd: &Path,
        registry: Option<Arc<Self::Registry>>,
        session: Self::Session,
        recorder: Arc<BusRecorder>,
        bridge: Option<Self::Bridge>,
        channels: Self::Channels,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
/// Complete startup host.
/// # Implementors
/// Every runtime supplying loading, frame and loop operations implements this trait.
pub trait Host: Runtime {}
impl<T: Runtime> Host for T {}
