use std::path::Path;
use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::input::{
    handle_backspace, handle_bus_c, handle_bus_g, handle_bus_slash, handle_char, handle_enter,
    handle_sessions_char,
};
use crate::tui::app::navigation::{
    handle_delete, handle_down, handle_end, handle_escape, handle_home, handle_left,
    handle_page_down, handle_page_up, handle_right, handle_symbol_enter, handle_tab, handle_up,
    toggle_help,
};
use crate::tui::app::state::App;
use crate::tui::app::symbols::symbol_search_active;
use crate::tui::models::ViewMode;
use crate::tui::worker_bridge::TuiWorkerBridge;

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
        KeyCode::Esc => handle_escape(app),
        KeyCode::Tab if app.state.slash_suggestions_visible() => handle_tab(app),
        KeyCode::Char('?') => toggle_help(app),
        KeyCode::Up => handle_up(app),
        KeyCode::Down => handle_down(app),
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
