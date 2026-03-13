use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::model_picker::open_model_picker;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::state::App;
use crate::tui::app::text::normalize_slash_command;
use crate::tui::models::ViewMode;

pub async fn handle_slash_command(
    app: &mut App,
    cwd: &std::path::Path,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    match normalize_slash_command(command).as_str() {
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
                "Protocol-first commands: /protocol /bus /model /sessions /swarm /ralph /symbols /settings /lsp /rlm /chat /new"
                    .to_string();
        }
        other => {
            app.state.status = format!("Unknown command: {other}");
        }
    }
}
