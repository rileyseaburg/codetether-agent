use std::io;
use std::sync::Arc;

use crossterm::{
    event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::AgentBus;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::event_loop::run_event_loop;
use crate::tui::app::message_text::sync_messages_from_session;
use crate::tui::app::session_sync::refresh_sessions;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(
            stdout,
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste
        );
    }
}

pub async fn run(project: Option<std::path::PathBuf>, allow_network: bool) -> anyhow::Result<()> {
    if allow_network {
        unsafe {
            std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1");
        }
    }

    if let Some(project) = project {
        std::env::set_current_dir(&project)?;
    }

    enable_raw_mode()?;
    let _guard = TerminalGuard;

    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let registry = ProviderRegistry::from_vault().await.ok().map(Arc::new);
    let cwd = std::env::current_dir().unwrap_or_default();
    let bus = AgentBus::new().into_arc();
    let mut bus_handle = bus.handle("tui");
    let worker_bridge = TuiWorkerBridge::spawn(None, None, None, Arc::clone(&bus))
        .await
        .ok()
        .flatten();

    let mut session = match Session::last_for_directory(Some(&cwd)).await {
        Ok(existing) => existing,
        Err(_) => Session::new().await?,
    };

    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(256);
    let (result_tx, result_rx) = mpsc::channel::<anyhow::Result<Session>>(8);

    let mut app = App::default();
    app.state.cwd_display = cwd.display().to_string();
    app.state.auto_apply_edits = session.metadata.auto_apply_edits;
    app.state.allow_network = session.metadata.allow_network || allow_network;
    app.state.slash_autocomplete = session.metadata.slash_autocomplete;
    app.state.session_id = Some(session.id.clone());
    session.metadata.allow_network = app.state.allow_network;
    sync_messages_from_session(&mut app, &session);
    refresh_sessions(&mut app, &cwd).await;
    if let Some(bridge) = worker_bridge.as_ref() {
        app.state
            .set_worker_bridge(bridge.worker_id.clone(), bridge.worker_name.clone());
        app.state.register_worker_agent("tui".to_string());
    }
    if let Err(err) = app.state.refresh_available_models(registry.as_ref()).await {
        app.state.status = format!("Failed to load models: {err}");
    }
    app.state.refresh_slash_suggestions();
    app.state.move_cursor_end();
    app.state.status = if app.state.messages.is_empty() {
        "Ready — type a message, or use /help for commands.".to_string()
    } else {
        "Loaded previous workspace session".to_string()
    };

    run_event_loop(
        &mut terminal,
        &mut app,
        &cwd,
        registry,
        &mut session,
        &mut bus_handle,
        worker_bridge,
        event_tx,
        event_rx,
        result_tx,
        result_rx,
    )
    .await?;

    terminal.show_cursor()?;
    Ok(())
}
