//! Terminal user-interface driver for CodeTether.
//!
//! This module coordinates startup for the interactive TUI: entering the target
//! project directory, configuring the terminal, starting collaboration services,
//! hydrating application state, and finally handing control to the event loop.
//! It intentionally keeps orchestration in one place while delegating terminal,
//! project, session, configuration, and event handling responsibilities to
//! focused submodules.

use crate::config::AccessMode;
use crate::session::Session;
use crate::tui::app::safe_draw::draw_ui;
use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;

/// Starts and runs the interactive terminal UI until the user exits.
///
/// The driver performs the full TUI lifecycle:
/// - enters the selected project directory with the requested network policy;
/// - switches the terminal into the runtime mode required by the TUI;
/// - starts the agent bus and optional A2A peer services;
/// - creates and hydrates the session-backed application state;
/// - draws the initial UI before slower startup work completes;
/// - loads workspace, configuration, registry, and worker bridge state;
/// - runs the event loop until shutdown;
/// - restores the cursor before returning.
///
/// # Parameters
///
/// * `project` - Optional project directory to enter before launching the TUI.
///   When `None`, the current working directory is used by the project entry
///   logic.
/// * `allow_network` - Whether startup and runtime components may enable
///   network-backed features such as peer collaboration and remote providers.
/// * `a2a_options` - Optional spawn configuration for starting or connecting to
///   an A2A peer for collaboration.
/// * `access_mode` - Optional access mode override applied to the session
///   configuration during startup.
/// * `yolo` - Full-auto mode: forces `AccessMode::Full` and auto-applies edits
///   without prompting.
///
/// # Returns
///
/// Returns `Ok(())` after the TUI exits cleanly and the terminal cursor has been
/// restored.
///
/// # Errors
///
/// Returns an error if project entry fails, terminal setup fails, session
/// creation fails, drawing fails, the event loop returns an error, or cursor
/// restoration fails.
///
/// # Side Effects
///
/// This function may change the process working directory, place the terminal in
/// TUI mode for the duration of the session, start background bus/peer services,
/// read configuration and workspace state from disk, and write session updates
/// through the event loop.
pub async fn run(
    project: Option<std::path::PathBuf>,
    allow_network: bool,
    a2a_options: Option<crate::a2a::spawn::SpawnOptions>,
    access_mode: Option<AccessMode>,
    yolo: bool,
) -> anyhow::Result<()> {
    super::project::enter(project, allow_network)?;
    let mut terminal_runtime = super::terminal::enter()?;
    let cwd = std::env::current_dir().unwrap_or_default();
    let bus = super::bus::start();
    let peer = super::peer::start(a2a_options, bus.clone()).await;
    let mut session = Session::new().await?.with_bus(bus.clone());
    let access_mode = super::full_auto::apply(&mut session, access_mode, yolo);
    let mut app = App::default();

    super::hydrate::initial(&mut app, &cwd, allow_network, peer.ready, &session);
    let view = SessionView::from_session(&session);
    draw_ui(&mut terminal_runtime.terminal, &mut app, &view)?;

    let mut startup = super::startup::load(&cwd, bus.clone()).await;
    super::config::apply_startup(&mut session, &mut startup, access_mode);
    let outcome = super::session_outcome::apply_load(&mut session, &bus, startup.session_load);
    super::hydrate::complete(
        &mut app,
        allow_network,
        &mut session,
        startup.worker_bridge.as_ref(),
        outcome,
        startup.workspace,
    );

    let mut bus_handle = bus.handle("tui");
    let channels = super::channels::session();
    super::loop_run::run(
        &mut terminal_runtime.terminal,
        &mut app,
        &cwd,
        startup.registry,
        session,
        &mut bus_handle,
        startup.worker_bridge,
        channels,
    )
    .await?;
    terminal_runtime.terminal.show_cursor()?;
    Ok(())
}
