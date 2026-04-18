use std::sync::Arc;

use crossterm::{
    event::{EnableBracketedPaste, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::{AgentBus, s3_sink::spawn_bus_s3_sink};
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::event_loop::run_event_loop;
use crate::tui::app::message_text::sync_messages_from_session;
use crate::tui::app::panic_cleanup::install_panic_cleanup_hook;
use crate::tui::app::state::App;
use crate::tui::app::terminal_state::{TerminalGuard, restore_terminal_state};
use crate::tui::ui::main::ui;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Outcome of trying to resume the most recent workspace session at startup.
///
/// Used to populate an informative status line so the user can distinguish
/// "no prior session existed" from "prior session loaded with 0 messages".
enum SessionLoadOutcome {
    Loaded {
        msg_count: usize,
        title: Option<String>,
    },
    NewFallback {
        reason: String,
    },
}

async fn init_tui_secrets_manager() {
    if crate::secrets::secrets_manager().is_some() {
        return;
    }

    match crate::secrets::SecretsManager::from_env().await {
        Ok(secrets_manager) => {
            if secrets_manager.is_connected() {
                tracing::info!("Connected to HashiCorp Vault for secrets management");
            }
            if let Err(err) = crate::secrets::init_from_manager(secrets_manager) {
                tracing::debug!(error = %err, "Secrets manager already initialized");
            }
        }
        Err(err) => {
            tracing::warn!(error = %err, "Vault not configured for TUI startup");
            tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
        }
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

    restore_terminal_state();
    enable_raw_mode()?;
    let _guard = TerminalGuard;
    let _panic_guard = install_panic_cleanup_hook();

    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let cwd = std::env::current_dir().unwrap_or_default();
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());
    spawn_bus_s3_sink(bus.clone());
    let mut session = Session::new().await?.with_bus(bus.clone());
    let mut app = App::default();
    app.state.cwd_display = cwd.display().to_string();
    app.state.allow_network = allow_network;
    app.state.session_id = Some(session.id.clone());
    app.state.status = "Loading providers and workspace…".to_string();
    terminal.draw(|f| ui(f, &mut app, &session))?;

    init_tui_secrets_manager().await;

    let registry = ProviderRegistry::from_vault().await.ok().map(Arc::new);
    let mut bus_handle = bus.handle("tui");
    let worker_bridge = TuiWorkerBridge::spawn(None, None, None, Arc::clone(&bus))
        .await
        .ok()
        .flatten();

    let (loaded_session, session_load_outcome) = match Session::last_for_directory(Some(&cwd)).await {
        Ok(existing) => {
            let msg_count = existing.messages.len();
            let title = existing.title.clone();
            (
                existing.with_bus(bus.clone()),
                SessionLoadOutcome::Loaded { msg_count, title },
            )
        }
        Err(err) => (
            Session::new().await?.with_bus(bus.clone()),
            SessionLoadOutcome::NewFallback {
                reason: err.to_string(),
            },
        ),
    };
    session = loaded_session;

    // Seed session metadata from the user's config so RLM settings
    // (threshold, iteration limits, subcall model) take effect.
    if let Ok(cfg) = crate::config::Config::load().await {
        session.apply_config(&cfg, None);
    }

    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(256);
    let (result_tx, result_rx) = mpsc::channel::<anyhow::Result<Session>>(8);

    app.state.workspace = crate::tui::models::WorkspaceSnapshot::capture(&cwd, 18);
    app.state.auto_apply_edits = session.metadata.auto_apply_edits;
    app.state.allow_network = session.metadata.allow_network || allow_network;
    app.state.slash_autocomplete = session.metadata.slash_autocomplete;
    app.state.use_worktree = session.metadata.use_worktree;
    app.state.session_id = Some(session.id.clone());
    session.metadata.allow_network = app.state.allow_network;
    sync_messages_from_session(&mut app, &session);
    if let Some(bridge) = worker_bridge.as_ref() {
        app.state
            .set_worker_bridge(bridge.worker_id.clone(), bridge.worker_name.clone());
        app.state.register_worker_agent("tui".to_string());
    }
    app.state.refresh_slash_suggestions();
    app.state.move_cursor_end();
    app.state.status = match &session_load_outcome {
        SessionLoadOutcome::Loaded { msg_count: 0, .. } => format!(
            "Loaded session {} (empty — type a message to start)",
            session.id
        ),
        SessionLoadOutcome::Loaded { msg_count, title } => {
            let label = title.as_deref().unwrap_or("(untitled)");
            format!("Loaded previous session: {label} — {msg_count} messages")
        }
        SessionLoadOutcome::NewFallback { reason } => {
            format!("New session (no prior session for this workspace: {reason})")
        }
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
