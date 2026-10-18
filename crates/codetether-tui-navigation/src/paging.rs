//! paging navigation policy.
use crate::subagent;
use crate::{Action, Host};
use codetether_tui_models::ViewMode;

/// Apply page up navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_page_up(app);
/// # }
/// ```
pub fn handle_page_up(app: &mut impl Host) {
    if app.context().show_help {
        app.apply(Action::HelpUp(10));
        return;
    }

    match app.context().view_mode {
        ViewMode::Subagents => subagent::page_up(app),
        ViewMode::Swarm if app.context().swarm_detail => app.apply(Action::SwarmDetailUp(10)),
        ViewMode::Ralph if app.context().ralph_detail => app.apply(Action::RalphDetailUp(10)),
        ViewMode::Bus if app.context().bus_detail => app.apply(Action::BusDetailUp(10)),
        ViewMode::FilePicker => app.apply(Action::FilePageUp),
        ViewMode::Chat => app.apply(Action::ChatPageUp),
        _ => {}
    }
}

/// Apply page down navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_page_down(app);
/// # }
/// ```
pub fn handle_page_down(app: &mut impl Host) {
    if app.context().show_help {
        app.apply(Action::HelpDown(10, 200));
        return;
    }

    match app.context().view_mode {
        ViewMode::Subagents => subagent::page_down(app),
        ViewMode::Swarm if app.context().swarm_detail => app.apply(Action::SwarmDetailDown(10)),
        ViewMode::Ralph if app.context().ralph_detail => app.apply(Action::RalphDetailDown(10)),
        ViewMode::Bus if app.context().bus_detail => app.apply(Action::BusDetailDown(10)),
        ViewMode::FilePicker => app.apply(Action::FilePageDown),
        ViewMode::Protocol => {
            app.apply(Action::ProtocolScroll(
                app.context().protocol_scroll.saturating_add(10),
            ));
        }
        ViewMode::Chat => app.apply(Action::ChatPageDown),
        _ => {}
    }
}
