//! View-aware Tab routing and swarm focus synchronization.
//!
//! Implement [Host] alongside the existing focus bindings, then pass key codes
//! to [dispatch]. Chat and subagent views use the shared focus policy; swarm
//! views move their selection and copy the selected agent into active focus.
use crate::focus;
use codetether_tui_models::ViewMode;
use crossterm::event::KeyCode;

/// Swarm selection operations required in addition to agent focus bindings.
///
/// # Implementors
/// Native applications bind these operations to their swarm list state.
pub trait Host: focus::Host {
    /// Move the swarm selection to the previous task, preserving wrap behavior.
    fn select_previous_swarm_task(&mut self);
    /// Move the swarm selection to the next task, preserving wrap behavior.
    fn select_next_swarm_task(&mut self);
}

/// Route Tab or BackTab according to the active view.
///
/// # Arguments
/// * `app` — Current view, focus, and swarm selection bindings.
/// * `code` — Key code supplied by the application's key dispatcher.
/// # Returns
/// Updates selection and focus in place; other views remain unchanged.
/// # Examples
/// ```no_run
/// # fn route(app: &mut impl codetether_tui_navigation::tab::Host) {
/// codetether_tui_navigation::tab::dispatch(app, crossterm::event::KeyCode::Tab);
/// # }
/// ```
pub fn dispatch(app: &mut impl Host, code: KeyCode) {
    if app.context().view_mode == ViewMode::Swarm {
        match code {
            KeyCode::BackTab => app.select_previous_swarm_task(),
            KeyCode::Tab => app.select_next_swarm_task(),
            _ => {}
        }
        let agent = app.selected_swarm_agent();
        app.set_active_agent(agent);
        return;
    }
    if app.context().view_mode != ViewMode::Chat && app.context().view_mode != ViewMode::Subagents {
        return;
    }
    match code {
        KeyCode::BackTab => focus::cycle_agent_focus_back(app),
        _ => focus::handle_tab(app),
    }
}
