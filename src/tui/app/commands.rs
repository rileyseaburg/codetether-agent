use std::path::Path;
use std::sync::Arc;

#[path = "commands/a2a.rs"]
mod a2a;
#[path = "commands/fork_session.rs"]
mod fork_session;
#[path = "commands/fork_view.rs"]
mod fork_view;
#[path = "commands/go_host.rs"]
mod go_host;
#[path = "commands/goal/mod.rs"]
pub(crate) mod goal;
#[path = "commands/open_editor.rs"]
mod open_editor;
#[path = "commands/ralph_host.rs"]
mod ralph_host;
#[path = "commands/ralph_registry.rs"]
mod ralph_registry;
#[path = "commands/ralph_view.rs"]
mod ralph_view;
#[path = "commands/session_ops.rs"]
mod session_ops;
#[path = "commands/settings_command.rs"]
mod settings_command;
use codetether_settings_policy::command::Setting;
#[path = "commands/undo_session.rs"]
mod undo_session;
#[path = "commands/undo_view.rs"]
mod undo_view;
use open_editor::open_editor;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::spawn_agent::handle_spawn_command;
use crate::tui::app::state::App;
use crate::tui::app::text::{
    command_with_optional_args, normalize_easy_command, normalize_slash_command,
};
use crate::tui::app::{codex_sessions, model_picker::open_model_picker};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

pub use crate::tui::app::auto_apply::{
    auto_apply_status_message, set_auto_apply_edits, toggle_auto_apply_edits,
};

fn push_system_message(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.scroll_to_bottom();
}
async fn handle_mcp_command(app: &mut App, raw: &str) {
    let output = codetether_mcp_command::execute(&app.state.mcp_registry, raw).await;
    app.state.status = output.status;
    if let Some(message) = output.message {
        push_system_message(app, message);
    }
}

/// Dispatch a `/context ...` command for context/RLM health status.
fn handle_context_command(app: &mut App, rest: &str) {
    match rest.trim() {
        "" | "status" => {
            let body = crate::tui::app::context_status::render(&app.state);
            push_system_message(app, body);
            app.state.status = "Context status".to_string();
        }
        _ => app.state.status = "Usage: /context [status]".to_string(),
    }
}

/// Undo the last `n` user turns in both the TUI and the persisted session.
///
/// `rest` may be empty (undo 1 turn) or a positive integer.
/// Each "turn" is one user message plus all assistant / tool messages that
/// followed it. Trailing system notices (e.g. a previous "Undid…" line) are
/// ignored when locating the cut point.
async fn handle_undo_command(app: &mut App, session: &mut Session, rest: &str) {
    codetether_turn_undo::command::execute(&mut undo_view::CommandView(app), session, rest).await;
}

/// Fork the current session: create a new session with a copy of the current
/// conversation (optionally truncated), switch the TUI to it, and leave the
/// original session untouched on disk.
///
/// Usage:
/// - `/fork` — fork at the current point (copy everything).
/// - `/fork N` — fork keeping only the first (total - N) turns (i.e. undo N
///   turns in the *fork* while leaving the parent intact).
async fn handle_fork_command(app: &mut App, _cwd: &Path, session: &mut Session, rest: &str) {
    codetether_session_fork::interactive::execute(&mut fork_view::CommandView(app), session, rest)
        .await;
}

/// Parse and dispatch a `/ralph ...` subcommand.
///
/// Returns `true` when the input matched a recognised subcommand (`run`,
/// `status`). Returns `false` for the bare `/ralph` (which falls through to
/// the "open monitor view" branch below).
async fn handle_ralph_subcommand(
    app: &mut App,
    cwd: &Path,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    rest: &str,
) -> bool {
    codetether_ralph_cli::interactive::handle::<ralph_host::AppHost, ProviderRegistry, _>(
        &mut ralph_view::CommandView(app),
        cwd,
        session.metadata.model.as_deref(),
        registry,
        rest,
    )
}

pub async fn handle_slash_command(
    app: &mut App,
    cwd: &std::path::Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    let normalized = normalize_easy_command(command);
    let normalized = normalize_slash_command(&normalized);

    if let Some(rest) = command_with_optional_args(&normalized, "/go") {
        handle_go_command(app, session, registry, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/image") {
        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
        crate::tui::app::input::attach_image_command(
            app,
            cwd,
            cleaned,
            codetether_image::interactive::Presentation::Chat,
        );
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/file") {
        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
        crate::tui::app::file_share::command(app, cwd, cleaned);
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/edit") {
        open_editor(
            app,
            cwd,
            rest.trim().trim_matches(|c| c == '"' || c == '\''),
        );
        return;
    }
    if let Some(rest) = command_with_optional_args(&normalized, "/autoapply") {
        settings_command::execute(app, session, Setting::AutoApply, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/network") {
        settings_command::execute(app, session, Setting::Network, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/autocomplete") {
        settings_command::execute(app, session, Setting::Autocomplete, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/ask") {
        let question = rest.trim();
        if question.is_empty() {
            app.state.status =
                "Usage: /ask <question> — ephemeral side question (full context, no tools, not saved)"
                    .to_string();
            push_system_message(
                app,
                "`/ask <question>` runs an ephemeral side query with full context but no tools, and is not saved to the session.",
            );
            return;
        }
        super::ask::run_ask(app, session, registry, question).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/context") {
        handle_context_command(app, rest);
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/mcp") {
        handle_mcp_command(app, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/ralph")
        && handle_ralph_subcommand(app, cwd, session, registry, rest).await
    {
        return;
    }
    // Fall through to the bare `/ralph` view-open handler below.

    // `/swarm <task>` launches a real swarm; bare `/swarm` falls through to the view.
    if let Some(rest) = command_with_optional_args(&normalized, "/swarm")
        && crate::tui::swarm_run::handle_swarm_command(app, session, rest)
    {
        return;
    }
    // `/forage [execute] [N]` launches an OKR-driven forage scan/execution.
    if session_ops::dispatch(app, cwd, session, &normalized).await {
        return;
    }

    match normalized.as_str() {
        "/help" => {
            app.state.show_help = true;
            app.state.help_scroll.offset = 0;
            app.state.status = "Help".to_string();
        }
        "/sessions" | "/session" => {
            refresh_sessions(app, cwd).await;
            app.state.clear_session_filter();
            app.state.set_view_mode(ViewMode::Sessions);
            app.state.status = "Session picker".to_string();
        }
        "/import-codex" => {
            codex_sessions::import_workspace_sessions(app, cwd).await;
        }
        "/swarm" => crate::tui::swarm_run::open_swarm_monitor(app),
        "/ralph" => {
            app.state
                .ralph
                .mark_active(app.state.cwd_display.clone(), "TUI Ralph monitor");
            app.state.set_view_mode(ViewMode::Ralph);
        }
        "/bus" | "/protocol" => {
            app.state.set_view_mode(ViewMode::Bus);
            app.state.status = "Protocol bus log".to_string();
        }
        "/model" => open_model_picker(app, session, registry).await,
        cmd @ ("/settings" | "/agents" | "/subagents") => {
            app.state.set_view_mode(if cmd == "/settings" {
                ViewMode::Settings
            } else {
                ViewMode::Subagents
            })
        }
        "/lsp" => app.state.set_view_mode(ViewMode::Lsp),
        "/rlm" => app.state.set_view_mode(ViewMode::Rlm),
        "/latency" => {
            app.state.set_view_mode(ViewMode::Latency);
            app.state.status = "Latency inspector".to_string();
        }
        "/transport" => {
            app.state.set_view_mode(ViewMode::Transport);
            app.state.status = "Transport health (TCP_INFO)".to_string();
        }
        "/inspector" => {
            app.state.set_view_mode(ViewMode::Inspector);
            app.state.status = "Inspector".to_string();
        }
        "/audit" => {
            crate::tui::audit_view::refresh_audit_snapshot(&mut app.state.audit).await;
            app.state.set_view_mode(ViewMode::Audit);
            app.state.status = "Audit — subagent activity".to_string();
        }
        "/git" => {
            let cwd = std::path::PathBuf::from(&app.state.cwd_display);
            app.state.git = crate::tui::git_capture::capture_git_state(&cwd);
            app.state.set_view_mode(ViewMode::Git);
            app.state.status = "Git status".to_string();
        }
        "/auditloop" => {
            app.state.set_view_mode(ViewMode::AuditLoop);
            app.state.status = "Audit loop".to_string();
        }
        "/chat" | "/home" | "/main" => return_to_chat(app),
        "/webview" => {
            app.state.chat_layout_mode =
                crate::tui::ui::webview::layout_mode::ChatLayoutMode::Webview;
            app.state.status = "Layout: Webview".to_string();
        }
        "/classic" => {
            app.state.chat_layout_mode =
                crate::tui::ui::webview::layout_mode::ChatLayoutMode::Classic;
            app.state.status = "Layout: Classic".to_string();
        }
        "/symbols" | "/symbol" => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        "/new" => crate::tui::app::session_sync::create_new_session(app, cwd, session).await,

        "/recall" => {
            let beliefs =
                crate::memory::palace::load_project_beliefs(Path::new(&app.state.cwd_display));
            if beliefs.is_empty() {
                push_system_message(
                    app,
                    "No project memories found. Memories accumulate as you work.",
                );
            } else {
                let active: Vec<&crate::cognition::beliefs::Belief> = beliefs
                    .iter()
                    .filter(|b| b.confidence >= 0.5)
                    .take(30)
                    .collect();
                let mut lines = vec![format!(
                    "🏰 Project Memory Palace ({} beliefs, {} active)",
                    beliefs.len(),
                    active.len()
                )];
                for b in &active {
                    lines.push(format!("  • {} [conf: {:.2}]", b.claim, b.confidence));
                }
                push_system_message(app, lines.join("\n"));
            }
            app.state.status = "Project memory palace".to_string();
        }
        "/plugin" => {
            push_system_message(app, "Usage: /plugin <install|list|remove> <name>");
        }
        "/keys" => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /context /access-mode /sandbox-mode /file /autoapply /network /autocomplete /mcp /model /sessions /import-codex /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new /undo /fork /spawn /kill /agents /agent /audit /git\nEasy aliases: /add /talk /list /remove /focus /home /say /ls /rm /main"
                    .to_string();
        }
        _ => {}
    }

    // --- commands with rest arguments handled below the simple match ---

    if let Some(rest) = command_with_optional_args(&normalized, "/spawn") {
        if rest.is_empty() {
            // No args → open interactive form
            crate::tui::app::spawn_form::open_spawn_form(app);
        } else {
            handle_spawn_command(app, session, cwd, rest).await;
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/kill") {
        crate::tui::app::managed_agent::command::admin::kill(app, session, rest).await;
        return;
    }

    if command_with_optional_args(&normalized, "/agents").is_some() {
        crate::tui::app::managed_agent::command::admin::list(app);
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/agent") {
        crate::tui::app::managed_agent::command::handle(app, session, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/autochat") {
        handle_autochat_command(app, rest);
        return;
    }

    // If we get here, none of the handlers above matched.
    // Easy-mode aliases are already normalized before reaching this point,
    if !matches!(
        normalized.as_str(),
        "/help"
            | "/sessions"
            | "/import-codex"
            | "/session"
            | "/swarm"
            | "/ralph"
            | "/bus"
            | "/protocol"
            | "/model"
            | "/settings"
            | "/lsp"
            | "/rlm"
            | "/latency"
            | "/transport"
            | "/audit"
            | "/git"
            | "/auditloop"
            | "/chat"
            | "/home"
            | "/main"
            | "/symbols"
            | "/symbol"
            | "/new"
            | "/undo"
            | "/keys"
            | "/file"
            | "/image"
            | "/autoapply"
            | "/network"
            | "/autocomplete"
            | "/mcp"
            | "/context"
            | "/spawn"
            | "/kill"
            | "/agents"
            | "/agent"
            | "/autochat"
            | "/protocols"
            | "/registry"
            | "/recall"
    ) {
        app.state.status = format!("Unknown command: {normalized}");
    }
}

async fn handle_go_command(
    app: &mut App,
    session: &mut Session,
    _registry: Option<&Arc<ProviderRegistry>>,
    rest: &str,
) {
    go_host::execute(app, session, rest).await;
}

fn handle_autochat_command(app: &mut App, rest: &str) {
    let task = rest.trim().to_string();
    if task.is_empty() {
        app.state.status = "Usage: /autochat <task description>".to_string();
        return;
    }
    if app.state.autochat.running {
        app.state.status = "Autochat relay already running.".to_string();
        return;
    }
    let model = app.state.last_completion_model.clone().unwrap_or_default();
    let rx = super::autochat::worker::start_autochat_relay(task, model);
    app.state.autochat.running = true;
    app.state.autochat.rx = Some(rx);
    app.state.status = "Autochat relay started.".to_string();
}
