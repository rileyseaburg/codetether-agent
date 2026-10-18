//! arrows navigation policy.
use crate::subagent;
use crate::{Action, Host};
use codetether_tui_models::ViewMode;
use crossterm::event::KeyModifiers;

/// Apply up navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// * `modifiers` — Keyboard modifiers; Control takes priority over Shift where applicable.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_up(app, crossterm::event::KeyModifiers::NONE);
/// # }
/// ```
pub fn handle_up(app: &mut impl Host, modifiers: KeyModifiers) {
    if app.context().show_help {
        app.apply(Action::HelpUp(1));
        return;
    }
    if app.context().symbols_active {
        app.apply(Action::SymbolPrevious);
        return;
    }
    if app.context().view_mode == ViewMode::Sessions {
        app.apply(Action::SessionPrevious);
        return;
    }
    if app.context().view_mode == ViewMode::Model {
        app.apply(Action::ModelPrevious);
        return;
    }
    if app.context().view_mode == ViewMode::Settings {
        app.apply(Action::SettingsPrevious);
        return;
    }
    if app.context().slash_navigable {
        app.apply(Action::SlashPrevious);
        return;
    }
    if app.context().view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.apply(Action::HistoryPrevious);
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            app.apply(Action::ToolUp(1));
        } else {
            app.apply(Action::ChatUp(1));
        }
        return;
    }

    match app.context().view_mode {
        ViewMode::Subagents => subagent::up(app),
        ViewMode::Swarm => app.apply(Action::SwarmPrevious),
        ViewMode::Ralph => {
            if app.context().ralph_detail {
                app.apply(Action::RalphDetailUp(1));
            } else {
                app.apply(Action::RalphPrevious);
            }
        }
        ViewMode::Bus if !app.context().bus_filter => {
            if app.context().bus_detail {
                app.apply(Action::BusDetailUp(1));
            } else {
                app.apply(Action::BusPrevious);
            }
        }
        ViewMode::FilePicker => app.apply(Action::FilePrevious),
        _ => app.apply(Action::ChatUp(1)),
    }
}

/// Apply down navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// * `modifiers` — Keyboard modifiers; Control takes priority over Shift where applicable.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_down(app, crossterm::event::KeyModifiers::NONE);
/// # }
/// ```
pub fn handle_down(app: &mut impl Host, modifiers: KeyModifiers) {
    if app.context().show_help {
        app.apply(Action::HelpDown(1, 200));
        return;
    }
    if app.context().symbols_active {
        app.apply(Action::SymbolNext);
        return;
    }
    if app.context().view_mode == ViewMode::Sessions {
        app.apply(Action::SessionNext);
        return;
    }
    if app.context().view_mode == ViewMode::Model {
        app.apply(Action::ModelNext);
        return;
    }
    if app.context().view_mode == ViewMode::Settings {
        app.apply(Action::SettingsNext);
        return;
    }
    if app.context().slash_navigable {
        app.apply(Action::SlashNext);
        return;
    }
    if app.context().view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.apply(Action::HistoryNext);
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            app.apply(Action::ToolDown(1));
        } else {
            app.apply(Action::ChatDown(1));
        }
        return;
    }

    match app.context().view_mode {
        ViewMode::Subagents => subagent::down(app),
        ViewMode::Swarm => app.apply(Action::SwarmNext),
        ViewMode::Ralph => {
            if app.context().ralph_detail {
                app.apply(Action::RalphDetailDown(1));
            } else {
                app.apply(Action::RalphNext);
            }
        }
        ViewMode::Bus if !app.context().bus_filter => {
            if app.context().bus_detail {
                app.apply(Action::BusDetailDown(1));
            } else {
                app.apply(Action::BusNext);
            }
        }
        ViewMode::FilePicker => app.apply(Action::FileNext),
        _ => app.apply(Action::ChatDown(1)),
    }
}
