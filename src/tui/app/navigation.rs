use crossterm::event::KeyModifiers;

use crate::tui::app::model_picker::close_model_picker;
use crate::tui::app::session_sync::return_to_chat;
use crate::tui::app::state::App;
use crate::tui::app::symbols::symbol_search_active;
use crate::tui::models::{InputMode, ViewMode};

pub fn handle_escape(app: &mut App) {
    if symbol_search_active(app) {
        app.state.symbol_search.close();
        app.state.status = "Closed symbol search".to_string();
    } else if app.state.show_help {
        app.state.show_help = false;
        app.state.status = "Closed help".to_string();
    } else if app.state.model_picker_active {
        close_model_picker(app);
        app.state.status = "Closed model picker".to_string();
    } else {
        match app.state.view_mode {
            ViewMode::Sessions => {
                app.state.clear_session_filter();
                return_to_chat(app);
            }
            ViewMode::FilePicker => {
                app.state.file_picker_active = false;
                return_to_chat(app);
            }
            ViewMode::Swarm if app.state.swarm.detail_mode => app.state.swarm.exit_detail(),
            ViewMode::Ralph if app.state.ralph.detail_mode => app.state.ralph.exit_detail(),
            ViewMode::Bus if app.state.bus_log.filter_input_mode => {
                app.state.bus_log.exit_filter_mode();
                app.state.status = "Protocol filter closed".to_string();
            }
            ViewMode::Bus if app.state.bus_log.detail_mode => app.state.bus_log.exit_detail(),
            ViewMode::Chat => app.state.input_mode = InputMode::Normal,
            _ => return_to_chat(app),
        }
    }
}

pub fn handle_tab(app: &mut App) {
    if app.state.apply_selected_slash_suggestion() {
        app.state.status = "Command autocompleted".to_string();
    }
}

pub fn toggle_help(app: &mut App) {
    app.state.show_help = !app.state.show_help;
    app.state.help_scroll.offset = 0;
    app.state.status = if app.state.show_help {
        "Help".to_string()
    } else {
        "Closed help".to_string()
    };
}

pub fn handle_up(app: &mut App, modifiers: KeyModifiers) {
    if app.state.show_help {
        app.state.help_scroll.scroll_up(1);
        return;
    }
    if symbol_search_active(app) {
        app.state.symbol_search.select_prev();
        return;
    }
    if app.state.view_mode == ViewMode::Sessions {
        app.state.sessions_select_prev();
        return;
    }
    if app.state.view_mode == ViewMode::Model {
        app.state.model_select_prev();
        return;
    }
    if app.state.view_mode == ViewMode::Settings {
        app.state.settings_select_prev();
        return;
    }
    if app.state.slash_suggestions_visible() {
        app.state.select_prev_slash_suggestion();
        return;
    }
    if app.state.view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            let _ = app.state.history_prev();
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            app.state.scroll_tool_preview_up(1);
        } else {
            app.state.scroll_up(1);
        }
        return;
    }

    match app.state.view_mode {
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state.swarm.detail_scroll_up(1);
            } else {
                app.state.swarm.select_prev();
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state.ralph.detail_scroll_up(1);
            } else {
                app.state.ralph.select_prev();
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state.bus_log.detail_scroll_up(1);
            } else {
                app.state.bus_log.select_prev();
            }
        }
        _ => app.state.scroll_up(1),
    }
}

pub fn handle_down(app: &mut App, modifiers: KeyModifiers) {
    if app.state.show_help {
        app.state.help_scroll.scroll_down(1, 200);
        return;
    }
    if symbol_search_active(app) {
        app.state.symbol_search.select_next();
        return;
    }
    if app.state.view_mode == ViewMode::Sessions {
        app.state.sessions_select_next();
        return;
    }
    if app.state.view_mode == ViewMode::Model {
        app.state.model_select_next();
        return;
    }
    if app.state.view_mode == ViewMode::Settings {
        app.state.settings_select_next();
        return;
    }
    if app.state.slash_suggestions_visible() {
        app.state.select_next_slash_suggestion();
        return;
    }
    if app.state.view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            let _ = app.state.history_next();
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            app.state.scroll_tool_preview_down(1);
        } else {
            app.state.scroll_down(1);
        }
        return;
    }

    match app.state.view_mode {
        ViewMode::Swarm => {
            if app.state.swarm.detail_mode {
                app.state.swarm.detail_scroll_down(1);
            } else {
                app.state.swarm.select_next();
            }
        }
        ViewMode::Ralph => {
            if app.state.ralph.detail_mode {
                app.state.ralph.detail_scroll_down(1);
            } else {
                app.state.ralph.select_next();
            }
        }
        ViewMode::Bus if !app.state.bus_log.filter_input_mode => {
            if app.state.bus_log.detail_mode {
                app.state.bus_log.detail_scroll_down(1);
            } else {
                app.state.bus_log.select_next();
            }
        }
        _ => app.state.scroll_down(1),
    }
}

pub fn handle_page_up(app: &mut App) {
    if app.state.show_help {
        app.state.help_scroll.scroll_up(10);
        return;
    }

    match app.state.view_mode {
        ViewMode::Swarm if app.state.swarm.detail_mode => app.state.swarm.detail_scroll_up(10),
        ViewMode::Ralph if app.state.ralph.detail_mode => app.state.ralph.detail_scroll_up(10),
        ViewMode::Bus if app.state.bus_log.detail_mode => app.state.bus_log.detail_scroll_up(10),
        ViewMode::Chat => app.state.scroll_up(10),
        _ => {}
    }
}

pub fn handle_page_down(app: &mut App) {
    if app.state.show_help {
        app.state.help_scroll.scroll_down(10, 200);
        return;
    }

    match app.state.view_mode {
        ViewMode::Swarm if app.state.swarm.detail_mode => app.state.swarm.detail_scroll_down(10),
        ViewMode::Ralph if app.state.ralph.detail_mode => app.state.ralph.detail_scroll_down(10),
        ViewMode::Bus if app.state.bus_log.detail_mode => app.state.bus_log.detail_scroll_down(10),
        ViewMode::Chat => app.state.scroll_down(10),
        _ => {}
    }
}

pub fn handle_home(app: &mut App) {
    if app.state.view_mode == ViewMode::Chat {
        app.state.move_cursor_home();
    }
}

pub fn handle_end(app: &mut App) {
    if app.state.view_mode == ViewMode::Chat {
        app.state.move_cursor_end();
    }
}

pub fn handle_left(app: &mut App, modifiers: KeyModifiers) {
    if app.state.view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.state.move_cursor_word_left();
        } else {
            app.state.move_cursor_left();
        }
    }
}

pub fn handle_right(app: &mut App, modifiers: KeyModifiers) {
    if app.state.view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.state.move_cursor_word_right();
        } else {
            app.state.move_cursor_right();
        }
    }
}

pub fn handle_delete(app: &mut App) {
    if app.state.view_mode == ViewMode::Chat {
        app.state.delete_forward();
    }
}

pub fn handle_symbol_enter(app: &mut App) {
    if let Some(symbol) = app.state.symbol_search.selected_symbol() {
        app.state.status = format!(
            "Selected symbol {} {}",
            symbol.name,
            symbol
                .line
                .map(|line| format!("at line {line}"))
                .unwrap_or_default()
        );
    }
    app.state.symbol_search.close();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_overlay_consumes_chat_arrow_navigation() {
        let mut app = App::default();
        app.state.show_help = true;
        app.state.set_view_mode(ViewMode::Chat);
        app.state.set_chat_max_scroll(25);
        app.state.scroll_to_bottom();

        handle_down(&mut app, KeyModifiers::NONE);

        assert_eq!(app.state.help_scroll.offset, 1);
        assert_eq!(app.state.chat_scroll, 1_000_000);
    }
}
