//! escape navigation policy.
use crate::subagent;
use crate::{Action, Host};
use codetether_tui_models::ViewMode;

/// Apply escape navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_escape(app);
/// # }
/// ```
pub fn handle_escape(app: &mut impl Host) {
    if app.context().symbols_active {
        app.apply(Action::CloseSearch);
        app.apply(Action::Status("Closed symbol search".to_string()));
    } else if app.context().show_help {
        app.apply(Action::SetHelp(false));
        app.apply(Action::Status("Closed help".to_string()));
    } else if app.context().model_picker_active {
        app.apply(Action::CloseModelPicker);
        app.apply(Action::Status("Closed model picker".to_string()));
    } else {
        match app.context().view_mode {
            ViewMode::Sessions => {
                app.apply(Action::ClearSessionFilter);
                app.apply(Action::ReturnToChat);
            }
            ViewMode::FilePicker => {
                if !app.file_picker_escape() {
                    app.apply(Action::DisableFilePicker);
                    app.apply(Action::ReturnToChat);
                }
            }
            ViewMode::Swarm if app.context().swarm_detail => app.apply(Action::SwarmExitDetail),
            ViewMode::Subagents => subagent::escape(app),
            ViewMode::Ralph if app.context().ralph_detail => app.apply(Action::RalphExitDetail),
            ViewMode::Bus if app.context().bus_filter => {
                app.apply(Action::BusExitFilter);
                app.apply(Action::Status("Protocol filter closed".to_string()));
            }
            ViewMode::Bus if app.context().bus_detail => app.apply(Action::BusExitDetail),
            ViewMode::Chat => app.apply(Action::NormalInput),
            _ => app.apply(Action::ReturnToChat),
        }
    }
}

/// Apply toggle help navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::toggle_help(app);
/// # }
/// ```
pub fn toggle_help(app: &mut impl Host) {
    app.apply(Action::SetHelp(!app.context().show_help));
    app.apply(Action::HelpReset);
    app.apply(Action::Status(if app.context().show_help {
        "Help".to_string()
    } else {
        "Closed help".to_string()
    }));
}
