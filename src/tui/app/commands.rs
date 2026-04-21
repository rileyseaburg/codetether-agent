use std::path::Path;
use std::sync::Arc;

use serde_json::Value;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::codex_sessions;
use crate::tui::app::file_share::attach_file_to_input;
use crate::tui::app::model_picker::open_model_picker;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::settings::{
    autocomplete_status_message, network_access_status_message, set_network_access,
    set_slash_autocomplete,
};
use crate::tui::app::state::{App, SpawnedAgent, agent_profile};
use crate::tui::app::text::{
    command_with_optional_args, normalize_easy_command, normalize_slash_command,
};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

fn auto_apply_flag_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

pub fn auto_apply_status_message(enabled: bool) -> String {
    format!("TUI edit auto-apply: {}", auto_apply_flag_label(enabled))
}

pub async fn set_auto_apply_edits(app: &mut App, session: &mut Session, next: bool) {
    app.state.auto_apply_edits = next;
    session.metadata.auto_apply_edits = next;

    match session.save().await {
        Ok(()) => {
            app.state.status = auto_apply_status_message(next);
        }
        Err(error) => {
            app.state.status = format!(
                "{} (not persisted: {error})",
                auto_apply_status_message(next)
            );
        }
    }
}

pub async fn toggle_auto_apply_edits(app: &mut App, session: &mut Session) {
    set_auto_apply_edits(app, session, !app.state.auto_apply_edits).await;
}

fn push_system_message(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.scroll_to_bottom();
}

async fn handle_mcp_command(app: &mut App, raw: &str) {
    let rest = raw.trim();
    if rest.is_empty() {
        app.state.status =
            "Usage: /mcp connect <name> <command...> | /mcp servers | /mcp tools [server] | /mcp call <server> <tool> [json]"
                .to_string();
        return;
    }

    if let Some(value) = rest.strip_prefix("connect ") {
        let mut parts = value.trim().splitn(2, char::is_whitespace);
        let Some(name) = parts.next().filter(|part| !part.is_empty()) else {
            app.state.status = "Usage: /mcp connect <name> <command...>".to_string();
            return;
        };
        let Some(command) = parts.next().map(str::trim).filter(|part| !part.is_empty()) else {
            app.state.status = "Usage: /mcp connect <name> <command...>".to_string();
            return;
        };

        match app.state.mcp_registry.connect(name, command).await {
            Ok(tool_count) => {
                app.state.status = format!("Connected MCP server '{name}' ({tool_count} tools)");
                push_system_message(
                    app,
                    format!("Connected MCP server `{name}` with {tool_count} tools."),
                );
            }
            Err(error) => {
                app.state.status = format!("MCP connect failed: {error}");
                push_system_message(app, format!("MCP connect failed for `{name}`: {error}"));
            }
        }
        return;
    }

    if rest == "servers" {
        let servers = app.state.mcp_registry.list_servers().await;
        if servers.is_empty() {
            app.state.status = "No MCP servers connected".to_string();
            push_system_message(app, "No MCP servers connected.");
        } else {
            app.state.status = format!("{} MCP server(s) connected", servers.len());
            let body = servers
                .into_iter()
                .map(|server| {
                    format!(
                        "- {} ({} tools) :: {}",
                        server.name, server.tool_count, server.command
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            push_system_message(app, format!("Connected MCP servers:\n{body}"));
        }
        return;
    }

    if let Some(value) = rest.strip_prefix("tools") {
        let server = value.trim();
        let server = if server.is_empty() {
            None
        } else {
            Some(server)
        };
        match app.state.mcp_registry.list_tools(server).await {
            Ok(tools) => {
                if tools.is_empty() {
                    app.state.status = "No MCP tools available".to_string();
                    push_system_message(app, "No MCP tools available.");
                } else {
                    app.state.status = format!("{} MCP tool(s) available", tools.len());
                    let body = tools
                        .into_iter()
                        .map(|(server_name, tool)| {
                            let description = tool
                                .description
                                .unwrap_or_else(|| "Remote MCP tool".to_string());
                            format!("- [{server_name}] {} — {}", tool.name, description)
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    push_system_message(app, format!("Available MCP tools:\n{body}"));
                }
            }
            Err(error) => {
                app.state.status = format!("MCP tools failed: {error}");
                push_system_message(app, format!("Failed to list MCP tools: {error}"));
            }
        }
        return;
    }

    if let Some(value) = rest.strip_prefix("call ") {
        let mut parts = value.trim().splitn(3, char::is_whitespace);
        let Some(server_name) = parts.next().filter(|part| !part.is_empty()) else {
            app.state.status = "Usage: /mcp call <server> <tool> [json]".to_string();
            return;
        };
        let Some(tool_name) = parts.next().filter(|part| !part.is_empty()) else {
            app.state.status = "Usage: /mcp call <server> <tool> [json]".to_string();
            return;
        };
        let arguments = match parts.next().map(str::trim).filter(|part| !part.is_empty()) {
            Some(raw_json) => match serde_json::from_str::<Value>(raw_json) {
                Ok(value) => value,
                Err(error) => {
                    app.state.status = format!("Invalid MCP JSON args: {error}");
                    return;
                }
            },
            None => Value::Object(Default::default()),
        };

        match app
            .state
            .mcp_registry
            .call_tool(server_name, tool_name, arguments)
            .await
        {
            Ok(output) => {
                app.state.status = format!("MCP tool finished: {server_name}/{tool_name}");
                push_system_message(
                    app,
                    format!("MCP `{server_name}` / `{tool_name}` result:\n{output}"),
                );
            }
            Err(error) => {
                app.state.status = format!("MCP call failed: {error}");
                push_system_message(
                    app,
                    format!("MCP `{server_name}` / `{tool_name}` failed: {error}"),
                );
            }
        }
        return;
    }

    app.state.status =
        "Usage: /mcp connect <name> <command...> | /mcp servers | /mcp tools [server] | /mcp call <server> <tool> [json]"
            .to_string();
}

/// Dispatch a `/goal ...` command — the human operator's manual entry
/// point for the session task log.
///
/// Subcommands:
/// - `/goal set <objective...>` — set the session objective.
/// - `/goal done [reason]` — clear the goal.
/// - `/goal reaffirm <note>` — record progress.
/// - `/goal show` (or bare `/goal`) — print goal + open tasks.
async fn handle_goal_command(app: &mut App, session: &Session, rest: &str) {
    use crate::session::tasks::{TaskEvent, TaskLog, TaskState, governance_block};
    use chrono::Utc;

    let log = match TaskLog::for_session(&session.id) {
        Ok(l) => l,
        Err(e) => {
            app.state.status = format!("/goal: {e}");
            return;
        }
    };

    let rest = rest.trim();
    let (verb, tail) = match rest.split_once(char::is_whitespace) {
        Some((v, t)) => (v, t.trim()),
        None => (rest, ""),
    };

    let event = match verb {
        "" | "show" | "status" => {
            let events = log.read_all().await.unwrap_or_default();
            let state = TaskState::from_log(&events);
            let text = governance_block(&state)
                .unwrap_or_else(|| "No goal and no tasks for this session.".to_string());
            push_system_message(app, text);
            app.state.status = "Goal shown".to_string();
            return;
        }
        "set" => {
            if tail.is_empty() {
                app.state.status = "Usage: /goal set <objective>".to_string();
                return;
            }
            TaskEvent::GoalSet {
                at: Utc::now(),
                objective: tail.to_string(),
                success_criteria: Vec::new(),
                forbidden: Vec::new(),
            }
        }
        "done" | "clear" => TaskEvent::GoalCleared {
            at: Utc::now(),
            reason: if tail.is_empty() { "completed".to_string() } else { tail.to_string() },
        },
        "reaffirm" => {
            if tail.is_empty() {
                app.state.status = "Usage: /goal reaffirm <progress note>".to_string();
                return;
            }
            TaskEvent::GoalReaffirmed { at: Utc::now(), progress_note: tail.to_string() }
        }
        other => {
            app.state.status =
                format!("Unknown /goal subcommand `{other}`. Try: set | done | reaffirm | show");
            return;
        }
    };

    match log.append(&event).await {
        Ok(()) => {
            let summary = match &event {
                TaskEvent::GoalSet { objective, .. } => format!("Goal set: {objective}"),
                TaskEvent::GoalCleared { reason, .. } => format!("Goal cleared: {reason}"),
                TaskEvent::GoalReaffirmed { progress_note, .. } => {
                    format!("Goal reaffirmed: {progress_note}")
                }
                _ => "Goal updated".to_string(),
            };
            push_system_message(app, summary.clone());
            app.state.status = summary;
        }
        Err(e) => {
            app.state.status = format!("/goal write failed: {e}");
        }
    }
}

/// Undo the last `n` user turns in both the TUI and the persisted session.
///
/// `rest` may be empty (undo 1 turn) or a positive integer.
/// Each "turn" is one user message plus all assistant / tool messages that
/// followed it. Trailing system notices (e.g. a previous "Undid…" line) are
/// ignored when locating the cut point.
async fn handle_undo_command(app: &mut App, session: &mut Session, rest: &str) {
    if app.state.processing {
        push_system_message(
            app,
            "Cannot undo while a response is in progress. Press Esc to cancel first.",
        );
        return;
    }

    let n: usize = match rest.trim() {
        "" => 1,
        s => match s.parse::<usize>() {
            Ok(v) if v >= 1 => v,
            _ => {
                app.state.status = "Usage: /undo [N] (N = how many turns to undo, default 1)"
                    .to_string();
                return;
            }
        },
    };

    // Collect indices of every User message in order; the Nth-from-the-end
    // is our cut point (truncate to it → drop that user message and
    // everything after).
    let session_user_idxs: Vec<usize> = session
        .messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| (m.role == crate::provider::Role::User).then_some(i))
        .collect();
    let tui_user_idxs: Vec<usize> = app
        .state
        .messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| matches!(m.message_type, MessageType::User).then_some(i))
        .collect();

    if session_user_idxs.is_empty() || tui_user_idxs.is_empty() {
        push_system_message(app, "Nothing to undo.");
        return;
    }

    let available = session_user_idxs.len().min(tui_user_idxs.len());
    let undo_count = n.min(available);

    // Index of the (available - undo_count)'th user message = first user turn to drop.
    let s_cut = session_user_idxs[available - undo_count];
    let t_cut = tui_user_idxs[available - undo_count];

    session.messages.truncate(s_cut);
    app.state.messages.truncate(t_cut);
    app.state.streaming_text.clear();
    app.state.scroll_to_bottom();

    if let Err(error) = session.save().await {
        tracing::warn!(error = %error, "Failed to save session after undo");
        app.state.status = format!("Undid {undo_count} turn(s) (not persisted: {error})");
    } else {
        app.state.status = format!("Undid {undo_count} turn(s)");
    }

    let partial_note = if undo_count < n {
        format!(" (only {undo_count} available)")
    } else {
        String::new()
    };
    push_system_message(
        app,
        format!("Undid {undo_count} turn(s){partial_note}."),
    );
}

/// Fork the current session: create a new session with a copy of the current
/// conversation (optionally truncated), switch the TUI to it, and leave the
/// original session untouched on disk.
///
/// Usage:
/// - `/fork` — fork at the current point (copy everything).
/// - `/fork N` — fork keeping only the first (total - N) turns (i.e. undo N
///   turns in the *fork* while leaving the parent intact).
async fn handle_fork_command(
    app: &mut App,
    _cwd: &Path,
    session: &mut Session,
    rest: &str,
) {
    if app.state.processing {
        push_system_message(
            app,
            "Cannot fork while a response is in progress. Press Esc to cancel first.",
        );
        return;
    }

    let drop_last_n: usize = match rest.trim() {
        "" => 0,
        s => match s.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                app.state.status =
                    "Usage: /fork [N] (drop last N user turns from the fork; default 0)"
                        .to_string();
                return;
            }
        },
    };

    // Persist current session first — fork must not lose the parent's tail.
    if let Err(error) = session.save().await {
        app.state.status = format!("Fork aborted: failed to save current session: {error}");
        return;
    }

    let parent_id = session.id.clone();

    // Build the child session.
    let mut child = match Session::new().await {
        Ok(s) => s,
        Err(err) => {
            app.state.status = format!("Fork failed: {err}");
            return;
        }
    };

    // Compute cut for the fork's copy.
    let session_user_idxs: Vec<usize> = session
        .messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| (m.role == crate::provider::Role::User).then_some(i))
        .collect();
    let session_cut = if drop_last_n == 0 || drop_last_n > session_user_idxs.len() {
        session.messages.len()
    } else {
        session_user_idxs[session_user_idxs.len() - drop_last_n]
    };

    let tui_user_idxs: Vec<usize> = app
        .state
        .messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| matches!(m.message_type, MessageType::User).then_some(i))
        .collect();
    let tui_cut = if drop_last_n == 0 || drop_last_n > tui_user_idxs.len() {
        app.state.messages.len()
    } else {
        tui_user_idxs[tui_user_idxs.len() - drop_last_n]
    };

    child.messages = session.messages[..session_cut].to_vec();
    child.metadata.auto_apply_edits = session.metadata.auto_apply_edits;
    child.metadata.allow_network = session.metadata.allow_network;
    child.metadata.slash_autocomplete = session.metadata.slash_autocomplete;
    child.metadata.use_worktree = session.metadata.use_worktree;
    child.metadata.model = session.metadata.model.clone();
    child.metadata.rlm = session.metadata.rlm.clone();
    child.title = session
        .title
        .as_ref()
        .map(|t| format!("{t} (fork)"))
        .or_else(|| Some("fork".to_string()));

    // Swap in the child session.
    let child_id = child.id.clone();
    *session = child;
    session.attach_global_bus_if_missing();

    if let Err(error) = session.save().await {
        app.state.status = format!("Fork created but failed to persist: {error}");
        return;
    }

    // Switch TUI state to the forked conversation.
    let forked_tui = app.state.messages[..tui_cut].to_vec();
    app.state.messages = forked_tui;
    app.state.session_id = Some(session.id.clone());
    app.state.streaming_text.clear();
    app.state.clear_request_timing();
    app.state.scroll_to_bottom();
    app.state.set_view_mode(ViewMode::Chat);

    let drop_note = if drop_last_n == 0 {
        String::new()
    } else {
        format!(" (dropped last {drop_last_n} turn(s) from fork)")
    };
    push_system_message(
        app,
        format!(
            "Forked session {}{}.\n  parent: {}\n  fork:   {}",
            &child_id[..8.min(child_id.len())],
            drop_note,
            parent_id,
            child_id,
        ),
    );
    app.state.status = format!("Forked → {}", &child_id[..8.min(child_id.len())]);
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
    let rest = rest.trim();
    if rest.is_empty() {
        // Bare `/ralph` — let caller open the monitor view.
        return false;
    }

    let mut parts = rest.split_whitespace();
    let verb = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    match verb {
        "run" => {
            let (prd_arg, max_iters) = parse_ralph_run_args(&args);

            let Some(registry) = registry.cloned() else {
                app.state.status = "Ralph run failed: no provider registry available".to_string();
                return true;
            };

            let prd_path = resolve_prd_path(cwd, prd_arg);
            if !prd_path.exists() {
                app.state.status = format!(
                    "Ralph run failed: PRD not found at {}",
                    prd_path.display()
                );
                return true;
            }

            let model_str = session
                .metadata
                .model
                .as_deref()
                .unwrap_or("claude-sonnet-4-5");
            let (provider, model) = match registry.resolve_model(model_str) {
                Ok(pair) => pair,
                Err(err) => {
                    app.state.status = format!("Ralph run failed: {err}");
                    return true;
                }
            };

            let (tx, rx) = tokio::sync::mpsc::channel(256);
            app.state.ralph.attach_event_rx(rx);
            app.state.set_view_mode(ViewMode::Ralph);
            app.state.status = format!(
                "Ralph running: {} (max {max_iters} iterations)",
                prd_path.display()
            );
            push_system_message(
                app,
                format!(
                    "Launching Ralph on `{}` via model `{model}` (max {max_iters} iterations).",
                    prd_path.display()
                ),
            );

            spawn_ralph_run(prd_path, provider, model, max_iters, tx);
            true
        }
        "status" => {
            let stories = &app.state.ralph.stories;
            if stories.is_empty() {
                app.state.status = "No Ralph run attached".to_string();
            } else {
                let passed = stories
                    .iter()
                    .filter(|s| matches!(s.status, crate::tui::ralph_view::RalphStoryStatus::Passed))
                    .count();
                app.state.status = format!(
                    "Ralph: {}/{} stories passed (iteration {}/{})",
                    passed,
                    stories.len(),
                    app.state.ralph.current_iteration,
                    app.state.ralph.max_iterations,
                );
            }
            true
        }
        _ => {
            app.state.status =
                "Usage: /ralph [run <prd.json> [--iters N]] | /ralph status".to_string();
            true
        }
    }
}

/// Parse positional / flag arguments for `/ralph run`.
///
/// Accepts: `[prd_path] [--iters N]` in either order.
fn parse_ralph_run_args<'a>(args: &[&'a str]) -> (Option<&'a str>, usize) {
    let mut prd: Option<&str> = None;
    let mut iters: usize = 10;
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--iters" | "--max-iterations" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    iters = v;
                    i += 2;
                    continue;
                }
            }
            other if !other.starts_with("--") && prd.is_none() => {
                prd = Some(other);
            }
            _ => {}
        }
        i += 1;
    }
    (prd, iters)
}

/// Resolve a user-provided PRD path against the working directory.
fn resolve_prd_path(cwd: &Path, arg: Option<&str>) -> std::path::PathBuf {
    let raw = arg.unwrap_or("prd.json");
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

/// Spawn a background task that drives a [`RalphLoop`] to completion and
/// streams events into the TUI through `event_tx`. The sender is dropped on
/// task exit, which the TUI notices via a disconnected `try_recv` and uses
/// to detach the receiver.
fn spawn_ralph_run(
    prd_path: std::path::PathBuf,
    provider: Arc<dyn crate::provider::Provider>,
    model: String,
    max_iters: usize,
    event_tx: tokio::sync::mpsc::Sender<crate::tui::ralph_view::RalphEvent>,
) {
    tokio::spawn(async move {
        use crate::ralph::{RalphConfig, RalphLoop};

        let config = RalphConfig {
            prd_path: prd_path.to_string_lossy().to_string(),
            max_iterations: max_iters,
            model: Some(model.clone()),
            ..Default::default()
        };

        let ralph = match RalphLoop::new(prd_path.clone(), provider, model, config).await {
            Ok(r) => r.with_event_tx(event_tx.clone()),
            Err(err) => {
                let _ = event_tx
                    .send(crate::tui::ralph_view::RalphEvent::Error(format!(
                        "Failed to initialise Ralph: {err}"
                    )))
                    .await;
                return;
            }
        };

        if let Err(err) = ralph.run().await {
            let _ = event_tx
                .send(crate::tui::ralph_view::RalphEvent::Error(format!(
                    "Ralph loop errored: {err}"
                )))
                .await;
        }
    });
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

    if let Some(rest) = command_with_optional_args(&normalized, "/image") {
        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
        if cleaned.is_empty() {
            app.state.status =
                "Usage: /image <path> (png, jpg, jpeg, gif, webp, bmp, svg).".to_string();
        } else {
            let path = Path::new(cleaned);
            let resolved = if path.is_absolute() {
                path.to_path_buf()
            } else {
                cwd.join(path)
            };
            match crate::tui::app::input::attach_image_file(&resolved) {
                Ok(attachment) => {
                    let display = resolved.display();
                    app.state.pending_images.push(attachment);
                    let count = app.state.pending_images.len();
                    app.state.status = format!(
                        "📷 Attached {display}. {count} image(s) pending. Press Enter to send."
                    );
                    push_system_message(
                        app,
                        format!(
                            "📷 Image attached: {display}. Type a message and press Enter to send."
                        ),
                    );
                }
                Err(msg) => {
                    push_system_message(app, format!("Failed to attach image: {msg}"));
                }
            }
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/file") {
        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
        if cleaned.is_empty() {
            app.state.status =
                "Usage: /file <path> (relative to workspace or absolute).".to_string();
        } else {
            attach_file_to_input(app, cwd, Path::new(cleaned));
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/autoapply") {
        let action = rest.trim().to_ascii_lowercase();
        let current = app.state.auto_apply_edits;
        let desired = match action.as_str() {
            "" | "toggle" => Some(!current),
            "status" => None,
            "on" | "true" | "yes" | "enable" | "enabled" => Some(true),
            "off" | "false" | "no" | "disable" | "disabled" => Some(false),
            _ => {
                app.state.status = "Usage: /autoapply [on|off|toggle|status]".to_string();
                return;
            }
        };

        if let Some(next) = desired {
            set_auto_apply_edits(app, session, next).await;
        } else {
            app.state.status = auto_apply_status_message(current);
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/network") {
        let current = app.state.allow_network;
        let desired = match rest.trim().to_ascii_lowercase().as_str() {
            "" | "toggle" => Some(!current),
            "status" => None,
            "on" | "true" | "yes" | "enable" | "enabled" => Some(true),
            "off" | "false" | "no" | "disable" | "disabled" => Some(false),
            _ => {
                app.state.status = "Usage: /network [on|off|toggle|status]".to_string();
                return;
            }
        };

        if let Some(next) = desired {
            set_network_access(app, session, next).await;
        } else {
            app.state.status = network_access_status_message(current);
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/autocomplete") {
        let current = app.state.slash_autocomplete;
        let desired = match rest.trim().to_ascii_lowercase().as_str() {
            "" | "toggle" => Some(!current),
            "status" => None,
            "on" | "true" | "yes" | "enable" | "enabled" => Some(true),
            "off" | "false" | "no" | "disable" | "disabled" => Some(false),
            _ => {
                app.state.status = "Usage: /autocomplete [on|off|toggle|status]".to_string();
                return;
            }
        };

        if let Some(next) = desired {
            set_slash_autocomplete(app, session, next).await;
        } else {
            app.state.status = autocomplete_status_message(current);
        }
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/ask") {
        super::ask::run_ask(app, session, registry, rest.trim()).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/mcp") {
        handle_mcp_command(app, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/ralph") {
        if handle_ralph_subcommand(app, cwd, session, registry, rest).await {
            return;
        }
        // Fall through to the bare `/ralph` view-open handler below.
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/goal") {
        handle_goal_command(app, session, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/undo") {
        handle_undo_command(app, session, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/fork") {
        handle_fork_command(app, cwd, session, rest).await;
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
        "/swarm" => {
            app.state.swarm.mark_active("TUI swarm monitor");
            app.state.set_view_mode(ViewMode::Swarm);
        }
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
        "/settings" => app.state.set_view_mode(ViewMode::Settings),
        "/lsp" => app.state.set_view_mode(ViewMode::Lsp),
        "/rlm" => app.state.set_view_mode(ViewMode::Rlm),
        "/latency" => {
            app.state.set_view_mode(ViewMode::Latency);
            app.state.status = "Latency inspector".to_string();
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
        "/new" => {
            // Create a fresh session so the old one is preserved on disk.
            match Session::new().await {
                Ok(mut new_session) => {
                    // Save the old session first — abort if persistence fails to
                    // avoid silently discarding the user's conversation.
                    if let Err(error) = session.save().await {
                        tracing::warn!(error = %error, "Failed to save current session before /new");
                        app.state.status = format!(
                            "Failed to save current session before creating new session: {error}"
                        );
                        return;
                    }

                    // Carry over user preferences into the new session.
                    new_session.metadata.auto_apply_edits = app.state.auto_apply_edits;
                    new_session.metadata.allow_network = app.state.allow_network;
                    new_session.metadata.slash_autocomplete = app.state.slash_autocomplete;
                    new_session.metadata.use_worktree = app.state.use_worktree;
                    new_session.metadata.model = session.metadata.model.clone();

                    *session = new_session;
                    session.attach_global_bus_if_missing();
                    if let Err(error) = session.save().await {
                        tracing::warn!(error = %error, "Failed to save new session");
                        app.state.status =
                            format!("New chat session created, but failed to persist: {error}");
                    } else {
                        app.state.status = "New chat session".to_string();
                    }
                    app.state.session_id = Some(session.id.clone());
                    app.state.messages.clear();
                    app.state.streaming_text.clear();
                    app.state.processing = false;
                    app.state.clear_request_timing();
                    app.state.scroll_to_bottom();
                    app.state.set_view_mode(ViewMode::Chat);
                    refresh_sessions(app, cwd).await;
                }
                Err(err) => {
                    app.state.status = format!("Failed to create new session: {err}");
                }
            }
        }
        "/keys" => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /file /autoapply /network /autocomplete /mcp /model /sessions /import-codex /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new /undo /fork /spawn /kill /agents /agent\nEasy aliases: /add /talk /list /remove /focus /home /say /ls /rm /main"
                    .to_string();
        }
        _ => {}
    }

    // --- commands with rest arguments handled below the simple match ---

    if let Some(rest) = command_with_optional_args(&normalized, "/spawn") {
        handle_spawn_command(app, rest).await;
        return;
    }

    if let Some(rest) = command_with_optional_args(&normalized, "/kill") {
        handle_kill_command(app, rest);
        return;
    }

    if command_with_optional_args(&normalized, "/agents").is_some() {
        handle_agents_command(app);
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
            | "/audit"
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
            | "/spawn"
            | "/kill"
            | "/agents"
            | "/agent"
            | "/autochat"
            | "/protocols"
            | "/registry"
    ) {
        app.state.status = format!("Unknown command: {normalized}");
    }
}

async fn handle_spawn_command(app: &mut App, rest: &str) {
    let rest = rest.trim();
    if rest.is_empty() {
        app.state.status = "Usage: /spawn <name> [instructions]".to_string();
        return;
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let Some(name) = parts.next().filter(|s| !s.is_empty()) else {
        app.state.status = "Usage: /spawn <name> [instructions]".to_string();
        return;
    };

    if app.state.spawned_agents.contains_key(name) {
        app.state.status = format!("Agent '{name}' already exists. Use /kill {name} first.");
        push_system_message(app, format!("Agent '{name}' already exists."));
        return;
    }

    let instructions = parts.next().unwrap_or("").trim().to_string();
    let profile = agent_profile(name);

    let system_prompt = if instructions.is_empty() {
        format!(
            "You are an AI assistant codenamed '{}' ({}) working as a sub-agent.
             Personality: {}
             Collaboration style: {}
             Signature move: {}",
            profile.codename,
            profile.profile,
            profile.personality,
            profile.collaboration_style,
            profile.signature_move,
        )
    } else {
        instructions.clone()
    };

    match Session::new().await {
        Ok(mut agent_session) => {
            agent_session.agent = format!("spawned:{}", name);
            agent_session.add_message(crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: system_prompt,
                }],
            });

            // Persist the agent session to disk so it can be recovered if
            // the TUI crashes before the agent sends its first prompt.
            if let Err(e) = agent_session.save().await {
                tracing::warn!(error = %e, "Failed to save spawned agent session");
            }

            let display_name = if instructions.is_empty() {
                format!("{} [{}]", name, profile.codename)
            } else {
                name.to_string()
            };

            app.state.spawned_agents.insert(
                name.to_string(),
                SpawnedAgent {
                    name: display_name.clone(),
                    instructions,
                    session: agent_session,
                    is_processing: false,
                },
            );

            app.state.status = format!("Spawned agent: {display_name}");
            push_system_message(
                app,
                format!(
                    "Spawned agent '{}' [{}] — ready for messages.",
                    name, profile.codename
                ),
            );
        }
        Err(error) => {
            app.state.status = format!("Failed to create agent session: {error}");
            push_system_message(app, format!("Failed to spawn agent '{name}': {error}"));
        }
    }
}

fn handle_kill_command(app: &mut App, rest: &str) {
    let name = rest.trim();
    if name.is_empty() {
        app.state.status = "Usage: /kill <name>".to_string();
        return;
    }

    if app.state.spawned_agents.remove(name).is_some() {
        if app.state.active_spawned_agent.as_deref() == Some(name) {
            app.state.active_spawned_agent = None;
        }
        app.state.streaming_agent_texts.remove(name);
        app.state.status = format!("Agent '{name}' removed.");
        push_system_message(app, format!("Agent '{name}' has been shut down."));
    } else {
        app.state.status = format!("Agent '{name}' not found.");
    }
}

fn handle_agents_command(app: &mut App) {
    if app.state.spawned_agents.is_empty() {
        app.state.status = "No spawned agents.".to_string();
        push_system_message(app, "No spawned agents. Use /spawn <name> to create one.");
    } else {
        let count = app.state.spawned_agents.len();
        let lines: Vec<String> = app
            .state
            .spawned_agents
            .iter()
            .map(|(key, agent)| {
                let msg_count = agent.session.history().len();
                let model = agent.session.metadata.model.as_deref().unwrap_or("default");
                let active = if app.state.active_spawned_agent.as_deref() == Some(key) {
                    " [active]"
                } else {
                    ""
                };
                format!(
                    "  {}{} — {} messages — model: {}",
                    agent.name, active, msg_count, model
                )
            })
            .collect();

        let body = lines.join(
            "
",
        );
        app.state.status = format!("{count} spawned agent(s)");
        push_system_message(
            app,
            format!(
                "Spawned agents ({count}):
{body}"
            ),
        );
    }
}
async fn handle_go_command(
    app: &mut App,
    session: &mut Session,
    _registry: Option<&Arc<ProviderRegistry>>,
    rest: &str,
) {
    use crate::tui::app::okr_gate::{PendingOkrApproval, ensure_okr_repository, next_go_model};
    use crate::tui::constants::AUTOCHAT_MAX_AGENTS;

    let task = rest.trim();
    if task.is_empty() {
        app.state.status = "Usage: /go <task description>".to_string();
        return;
    }

    // Rotate model for /go
    let current_model = session.metadata.model.as_deref();
    let model = next_go_model(current_model);
    session.metadata.model = Some(model.clone());
    if let Err(error) = session.save().await {
        tracing::warn!(error = %error, "Failed to save session after model swap");
    }

    // Initialize OKR repository if needed
    ensure_okr_repository(&mut app.state.okr_repository).await;

    // Draft OKR and present for approval
    let pending = PendingOkrApproval::propose(task.to_string(), AUTOCHAT_MAX_AGENTS, model).await;

    push_system_message(app, pending.approval_prompt());

    app.state.pending_okr_approval = Some(pending);
    app.state.status = "OKR draft awaiting approval \u{2014} [A]pprove or [D]eny".to_string();
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
