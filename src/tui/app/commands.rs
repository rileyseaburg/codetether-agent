use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::file_share::attach_file_to_input;
use crate::tui::app::model_picker::open_model_picker;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::state::App;
use crate::tui::app::text::{command_with_optional_args, normalize_slash_command};
use crate::tui::models::ViewMode;

pub async fn handle_slash_command(
    app: &mut App,
    cwd: &std::path::Path,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    let normalized = normalize_slash_command(command);

    if let Some(rest) = command_with_optional_args(&normalized, "/file") {
        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
        if cleaned.is_empty() {
            app.state.status = "File picker is not extracted yet. Use /file <path>.".to_string();
        } else {
            attach_file_to_input(app, cwd, Path::new(cleaned));
        }
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
        "/swarm" => app.state.set_view_mode(ViewMode::Swarm),
        "/ralph" => app.state.set_view_mode(ViewMode::Ralph),
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
        "/keys" => {
            app.state.status =
                "Protocol-first commands: /protocol /bus /file /model /sessions /swarm /ralph /latency /symbols /settings /lsp /rlm /chat /new"
                    .to_string();
        }
        other => {
            app.state.status = format!("Unknown command: {other}");
        }
    }
}
