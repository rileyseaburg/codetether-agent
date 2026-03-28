use std::path::Path;
use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::commands::toggle_auto_apply_edits;
use crate::tui::app::input::{
    handle_backspace, handle_bus_c, handle_bus_g, handle_bus_slash, handle_char, handle_enter,
    handle_paste, handle_sessions_char,
};
use crate::tui::app::navigation::{
    handle_delete, handle_down, handle_end, handle_escape, handle_home, handle_left,
    handle_page_down, handle_page_up, handle_right, handle_symbol_enter, handle_tab, handle_up,
    toggle_help,
};
use crate::tui::app::settings::{toggle_network_access, toggle_slash_autocomplete};
use crate::tui::app::state::App;
use crate::tui::app::symbols::symbol_search_active;
use crate::tui::models::ViewMode;
use crate::tui::worker_bridge::TuiWorkerBridge;

const MOUSE_WHEEL_SCROLL_AMOUNT: usize = 3;

pub async fn handle_event(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }

    match key.code {
        KeyCode::Char('c')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            return Ok(true);
        }
        KeyCode::Char('q')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            return Ok(true);
        }
        KeyCode::Char('t')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        KeyCode::Char('v')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL)
                && app.state.view_mode == ViewMode::Chat =>
        {
            if let Some(image) = crate::tui::app::input::get_clipboard_image() {
                app.state.pending_images.push(image);
                let image_count = app.state.pending_images.len();
                app.state.status = if image_count == 1 {
                    "Attached 1 clipboard image. Type a message and press Enter to send."
                        .to_string()
                } else {
                    format!("Attached {image_count} clipboard images. Press Enter to send them.")
                };
            }
        }
        KeyCode::Esc => handle_escape(app),
        KeyCode::Tab if app.state.slash_suggestions_visible() => handle_tab(app),
        KeyCode::Char('?') => toggle_help(app),
        KeyCode::Char('j')
            if key.modifiers.contains(KeyModifiers::ALT)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_down(1);
        }
        KeyCode::Char('k')
            if key.modifiers.contains(KeyModifiers::ALT)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_up(1);
        }
        KeyCode::Char('d')
            if key.modifiers.contains(KeyModifiers::ALT)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_down(5);
        }
        KeyCode::Char('u')
            if key.modifiers.contains(KeyModifiers::ALT)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_up(5);
        }
        KeyCode::Char('g')
            if key.modifiers.contains(KeyModifiers::CONTROL)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_to_top();
        }
        KeyCode::Char('G')
            if key.modifiers.contains(KeyModifiers::CONTROL)
                && app.state.view_mode == ViewMode::Chat =>
        {
            app.state.scroll_to_bottom();
        }
        KeyCode::Up => handle_up(app, key.modifiers),
        KeyCode::Down => handle_down(app, key.modifiers),
        KeyCode::PageUp => handle_page_up(app),
        KeyCode::PageDown => handle_page_down(app),
        KeyCode::Home => handle_home(app),
        KeyCode::End => handle_end(app),
        KeyCode::Left => handle_left(app, key.modifiers),
        KeyCode::Right => handle_right(app, key.modifiers),
        KeyCode::Delete => handle_delete(app),
        KeyCode::Enter if symbol_search_active(app) => handle_symbol_enter(app),
        KeyCode::Enter => {
            handle_enter(
                app,
                cwd,
                session,
                registry,
                worker_bridge,
                event_tx,
                result_tx,
            )
            .await;
        }
        KeyCode::Char('a') if app.state.view_mode == ViewMode::Settings => {
            toggle_auto_apply_edits(app, session).await;
        }
        KeyCode::Char('n') if app.state.view_mode == ViewMode::Settings => {
            toggle_network_access(app, session).await;
        }
        KeyCode::Tab if app.state.view_mode == ViewMode::Settings => {
            toggle_slash_autocomplete(app, session).await;
        }
        KeyCode::Backspace if app.state.view_mode == ViewMode::Sessions => {
            app.state.session_filter_backspace();
        }
        KeyCode::Backspace => handle_backspace(app).await,
        KeyCode::Char('g') if app.state.view_mode == ViewMode::Bus => handle_bus_g(app),
        KeyCode::Char('c') if app.state.view_mode == ViewMode::Bus => handle_bus_c(app),
        KeyCode::Char('/') if app.state.view_mode == ViewMode::Bus => handle_bus_slash(app),
        KeyCode::Char(c) if app.state.view_mode == ViewMode::Sessions => {
            handle_sessions_char(app, key.modifiers, c)
        }
        KeyCode::Char(c) => handle_char(app, key.modifiers, c).await,
        _ => {}
    }

    Ok(false)
}

pub async fn handle_paste_event(app: &mut App, text: &str) {
    handle_paste(app, text).await;
}

pub fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => scroll_mouse_up(app),
        MouseEventKind::ScrollDown => scroll_mouse_down(app),
        _ => {}
    }
}

fn scroll_mouse_up(app: &mut App) {
    if app.state.show_help {
        app.state.help_scroll.scroll_up(MOUSE_WHEEL_SCROLL_AMOUNT);
        return;
    }
    if symbol_search_active(app) {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.symbol_search.select_prev();
        }
        return;
    }
    if app.state.view_mode == ViewMode::Sessions {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.sessions_select_prev();
        }
        return;
    }
    if app.state.view_mode == ViewMode::Model {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.model_select_prev();
        }
        return;
    }
    if app.state.slash_suggestions_visible() {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.select_prev_slash_suggestion();
        }
        return;
    }

    match app.state.view_mode {
        ViewMode::Chat => app.state.scroll_up(MOUSE_WHEEL_SCROLL_AMOUNT),
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state.swarm.detail_scroll_up(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.swarm.select_prev();
                }
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state.ralph.detail_scroll_up(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.ralph.select_prev();
                }
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state
                    .bus_log
                    .detail_scroll_up(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.bus_log.select_prev();
                }
            }
        }
        ViewMode::Settings
        | ViewMode::Lsp
        | ViewMode::Rlm
        | ViewMode::Latency        | ViewMode::Protocol        | ViewMode::Bus
        | ViewMode::Sessions
        | ViewMode::Model => {}
    }
}

fn scroll_mouse_down(app: &mut App) {
    if app.state.show_help {
        app.state
            .help_scroll
            .scroll_down(MOUSE_WHEEL_SCROLL_AMOUNT, 200);
        return;
    }
    if symbol_search_active(app) {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.symbol_search.select_next();
        }
        return;
    }
    if app.state.view_mode == ViewMode::Sessions {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.sessions_select_next();
        }
        return;
    }
    if app.state.view_mode == ViewMode::Model {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.model_select_next();
        }
        return;
    }
    if app.state.slash_suggestions_visible() {
        for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
            app.state.select_next_slash_suggestion();
        }
        return;
    }

    match app.state.view_mode {
        ViewMode::Chat => app.state.scroll_down(MOUSE_WHEEL_SCROLL_AMOUNT),
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state
                    .swarm
                    .detail_scroll_down(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.swarm.select_next();
                }
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state
                    .ralph
                    .detail_scroll_down(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.ralph.select_next();
                }
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state
                    .bus_log
                    .detail_scroll_down(MOUSE_WHEEL_SCROLL_AMOUNT);
            } else {
                for _ in 0..MOUSE_WHEEL_SCROLL_AMOUNT {
                    app.state.bus_log.select_next();
                }
            }
        }
        ViewMode::Settings
        | ViewMode::Lsp
        | ViewMode::Rlm
        | ViewMode::Latency
        | ViewMode::Protocol
        | ViewMode::Bus
        | ViewMode::Sessions
        | ViewMode::Model => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_wheel_scrolls_chat_from_follow_latest() {
        let mut app = App::default();
        app.state.set_chat_max_scroll(25);
        app.state.scroll_to_bottom();

        handle_mouse_event(
            &mut app,
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                column: 0,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
        );

        assert_eq!(app.state.chat_scroll, 22);
    }
}
