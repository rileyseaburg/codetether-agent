use std::path::Path;
use std::sync::Arc;

use serde_json::Value;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::file_share::attach_file_to_input;
use crate::tui::app::model_picker::open_model_picker;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::settings::{
    autocomplete_status_message, network_access_status_message, set_network_access,
    set_slash_autocomplete,
};
use crate::tui::app::state::{agent_profile, App, SpawnedAgent};
use crate::tui::app::text::{command_with_optional_args, normalize_slash_command};
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

pub async fn handle_slash_command(
    app: &mut App,
    cwd: &std::path::Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    let normalized = normalize_slash_command(command);

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

    if let Some(rest) = command_with_optional_args(&normalized, "/mcp") {
        handle_mcp_command(app, rest).await;
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
        "/chat" | "/home" => return_to_chat(app),
        "/symbols" | "/symbol" => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        "/new" => {
            app.state.messages.clear();
            app.state.chat_scroll = 0;
            app.state.status = "New chat buffer".to_string();
            app.state.set_view_mode(ViewMode::Chat);
        }
        "/undo" => {
            // Remove from TUI messages: walk backwards removing everything
            // until we've removed the last user message (inclusive)
            let mut found_user = false;
            while let Some(msg) = app.state.messages.last() {
                if matches!(msg.message_type, MessageType::User) {
                    if found_user {
                        break; // hit the previous user turn, stop
                    }
                    found_user = true;
                }
                // Stop if we hit a system message before finding a user message
                if matches!(msg.message_type, MessageType::System) && !found_user {
                    break;
                }
                app.state.messages.pop();
            }

            if !found_user {
                push_system_message(app, "Nothing to undo.");
                return;
            }

            // Remove from session: walk backwards removing the last user message
            // and all assistant/tool messages after it
            let mut found_session_user = false;
            while let Some(msg) = session.messages.last() {
                if msg.role == crate::provider::Role::User {
                    if found_session_user {
                        break;
                    }
                    found_session_user = true;
                }
                if msg.role == crate::provider::Role::System && !found_session_user {
                    break;
                }
                session.messages.pop();
            }
            if let Err(error) = session.save().await {
                tracing::warn!(error = %error, "Failed to save session after undo");
            }

            push_system_message(app, "Undid last message and response.");
        }
        "/keys" => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /file /autoapply /network /autocomplete /mcp /model /sessions /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new /undo /spawn /kill /agents"
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

    // If we get here, none of the handlers above matched
    if !matches!(
        normalized.as_str(),
        "/help"
            | "/sessions"
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
            | "/chat"
            | "/home"
            | "/symbols"
            | "/symbol"
            | "/new"
            | "/undo"
            | "/keys"
            | "/file"
            | "/autoapply"
            | "/network"
            | "/autocomplete"
            | "/mcp"
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
            profile.codename, profile.profile, profile.personality,
            profile.collaboration_style, profile.signature_move,
        )
    } else {
        instructions.clone()
    };

    match Session::new().await {
        Ok(mut agent_session) => {
            agent_session.agent = format!("spawned:{}", name);
            agent_session.messages.push(crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text { text: system_prompt }],
            });

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
                format!("Spawned agent '{}' [{}] — ready for messages.", name, profile.codename),
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
                let msg_count = agent.session.messages.len();
                let model = agent.session.metadata.model.as_deref().unwrap_or("default");
                let active = if app.state.active_spawned_agent.as_deref() == Some(key) {
                    " [active]"
                } else {
                    ""
                };
                format!("  {}{} — {} messages — model: {}", agent.name, active, msg_count, model)
            })
            .collect();

        let body = lines.join("
");
        app.state.status = format!("{count} spawned agent(s)");
        push_system_message(app, format!("Spawned agents ({count}):
{body}"));
    }
}