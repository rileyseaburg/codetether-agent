use std::sync::Arc;

use crossterm::{
    event::{EnableBracketedPaste, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
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
use crate::tui::app::resume_window::session_resume_window;
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
        dropped: usize,
        file_bytes: u64,
        /// When `Some`, the resumed session was truncated and has been
        /// forked to a new UUID; this is the original on-disk id that
        /// remains intact with the full history.
        original_id: Option<String>,
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

pub async fn run(
    project: Option<std::path::PathBuf>,
    allow_network: bool,
    a2a_options: Option<crate::a2a::spawn::SpawnOptions>,
) -> anyhow::Result<()> {
    if allow_network {
        unsafe {
            std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1");
        }
    }

    if let Some(project) = project {
        // Validate with a clear error before touching process state — the
        // bare `set_current_dir` error on Windows is "The system cannot
        // find the file specified. (os error 2)", which is opaque.
        let project = project.as_path();
        if !project.exists() {
            anyhow::bail!(
                "project directory does not exist: {}\n\
                 hint: `tui` takes an optional path to an existing workspace, \
                 not a subcommand. Run `codetether tui` from inside your project, \
                 or pass a directory that already exists.",
                project.display(),
            );
        }
        if !project.is_dir() {
            anyhow::bail!("project path is not a directory: {}", project.display(),);
        }
        std::env::set_current_dir(project).map_err(|e| {
            anyhow::anyhow!(
                "failed to enter project directory {}: {e}",
                project.display(),
            )
        })?;
    }

    restore_terminal_state();
    enable_raw_mode()?;
    let _guard = TerminalGuard;
    let _panic_guard = install_panic_cleanup_hook();

    let mut stdout = std::io::stdout();
    // NOTE: We intentionally do NOT enable mouse capture. Capturing
    // mouse events breaks native terminal text selection (users can't
    // click-drag to select chat output and copy it). Keyboard scrolling
    // via ↑↓ / PageUp / PageDown is already bound. Hold the
    // terminal-specific modifier (Shift on most emulators, Option/Alt
    // on macOS Terminal.app) during drag if mouse capture is ever
    // re-enabled in the future.
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    // Request the kitty keyboard protocol so terminals that support it
    // (kitty, foot, WezTerm, Ghostty, modern Konsole, Alacritty ≥ 0.13)
    // report modifier bits on Enter, enabling Shift+Enter to insert a
    // newline in chat input instead of being indistinguishable from
    // plain Enter. Failure is non-fatal — Alt+Enter still works on
    // dumber terminals and we fall back to bracketed paste for
    // multi-line input.
    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let cwd = std::env::current_dir().unwrap_or_default();
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());
    spawn_bus_s3_sink(bus.clone());

    // Optional: start an A2A peer endpoint inside the TUI process so other
    // agents (TUIs, `codetether spawn` peers, plain curl) can reach this
    // session over the A2A wire protocol. Inbound `message/send` requests
    // are answered by a fresh background session — they do not appear in
    // the user's interactive TUI conversation. See docs/a2a-spawn.md.
    //
    // LIFETIME: the binding below MUST live for the duration of the TUI
    // event loop. `A2APeerHandle::Drop` aborts the background server and
    // discovery tasks, so an early drop kills the peer. The leading `_`
    // is the unused-name convention (the variable is read by Drop, not by
    // any code), NOT the `_` discard pattern — those have different drop
    // semantics. Do not change this to `let _ = ...`.
    let _a2a_peer_lifetime_guard = if let Some(opts) = a2a_options {
        match crate::a2a::spawn::start_a2a_in_background(opts, bus.clone()).await {
            Ok(handle) => {
                tracing::info!(
                    agent = %handle.agent_name,
                    bind_addr = %handle.bind_addr,
                    public_url = %handle.public_url,
                    "TUI A2A peer endpoint ready"
                );
                Some(handle)
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to start TUI A2A peer; continuing without it");
                None
            }
        }
    } else {
        None
    };

    let mut session = Session::new().await?.with_bus(bus.clone());
    let mut app = App::default();
    app.state.cwd_display = cwd.display().to_string();
    app.state.allow_network = allow_network;
    app.state.session_id = Some(session.id.clone());
    app.state.status = "Loading providers and workspace…".to_string();
    terminal.draw(|f| ui(f, &mut app, &session))?;

    let registry_task = async {
        init_tui_secrets_manager().await;
        ProviderRegistry::from_vault().await.ok().map(Arc::new)
    };
    let worker_bridge_task = TuiWorkerBridge::spawn(None, None, None, Arc::clone(&bus));
    // Hard cap the session scan so a workspace full of old/huge session
    // files can never block the TUI from coming up. If the scan exceeds
    // the budget we start a fresh session; the scan task is detached and
    // its result (if any) is simply dropped.
    const SESSION_SCAN_BUDGET: std::time::Duration = std::time::Duration::from_secs(3);
    let resume_window = session_resume_window();
    let session_task = tokio::time::timeout(
        SESSION_SCAN_BUDGET,
        Session::last_for_directory_tail(Some(&cwd), resume_window),
    );
    let config_task = crate::config::Config::load();
    let workspace_task = tokio::task::spawn_blocking({
        let cwd = cwd.clone();
        move || crate::tui::models::WorkspaceSnapshot::capture(&cwd, 18)
    });

    let (registry, worker_bridge_result, session_timeout_result, config, workspace_snapshot) = tokio::join!(
        registry_task,
        worker_bridge_task,
        session_task,
        config_task,
        workspace_task,
    );

    let loaded_session = match session_timeout_result {
        Ok(inner) => inner,
        Err(_) => {
            tracing::warn!(
                budget_secs = SESSION_SCAN_BUDGET.as_secs(),
                "session scan exceeded budget; starting fresh session",
            );
            Err(anyhow::anyhow!(
                "session scan timed out after {}s",
                SESSION_SCAN_BUDGET.as_secs(),
            ))
        }
    };

    let worker_bridge = worker_bridge_result.ok().flatten();
    let mut bus_handle = bus.handle("tui");
    let session_load_outcome = match loaded_session {
        Ok(load) => {
            let title = load.session.title.clone();
            let dropped = load.dropped;
            let file_bytes = load.file_bytes;
            let original_id = load.session.id.clone();
            session = load.session.with_bus(bus.clone());
            // If the on-disk transcript was truncated to fit in memory,
            // FORK to a new UUID so a later `.save()` cannot clobber the
            // full-history file on disk with our shortened window.
            if dropped > 0 {
                let new_id = uuid::Uuid::new_v4().to_string();
                tracing::warn!(
                    original_id = %original_id,
                    new_id = %new_id,
                    dropped,
                    file_bytes,
                    "forked large session on resume to protect on-disk history",
                );
                session.id = new_id;
                session.title = Some(format!(
                    "{} (continued)",
                    title.as_deref().unwrap_or("large session"),
                ));
            }
            let msg_count = session.history().len();
            SessionLoadOutcome::Loaded {
                msg_count,
                title,
                dropped,
                file_bytes,
                original_id: if dropped > 0 { Some(original_id) } else { None },
            }
        }
        Err(err) => SessionLoadOutcome::NewFallback {
            reason: err.to_string(),
        },
    };

    if let Ok(cfg) = config {
        session.apply_config(&cfg, registry.as_deref());
    }

    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(256);
    let (result_tx, result_rx) = mpsc::channel::<anyhow::Result<Session>>(8);

    app.state.workspace = workspace_snapshot.unwrap_or_else(|err| {
        tracing::warn!(error = %err, "Workspace snapshot task failed");
        crate::tui::models::WorkspaceSnapshot::capture(&cwd, 18)
    });
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
        SessionLoadOutcome::Loaded {
            msg_count,
            title,
            dropped,
            file_bytes,
            original_id,
        } => {
            let label = title.as_deref().unwrap_or("(untitled)");
            if *dropped > 0 {
                let mb = *file_bytes as f64 / (1024.0 * 1024.0);
                let orig = original_id.as_deref().unwrap_or("?");
                format!(
                    "⚠ Large session ({mb:.1} MiB): showing last {msg_count} of {total} entries from \"{label}\". Forked to a new session — original {orig} preserved on disk.",
                    total = msg_count + *dropped,
                )
            } else {
                format!("Loaded previous session: {label} — {msg_count} messages")
            }
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
