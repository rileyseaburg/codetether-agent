//! Swarm arrow routing and modal-priority regression coverage.
use codetether_tui_models::ViewMode;
use codetether_tui_navigation::{
    Action, Context, Host, handle_down, handle_left, handle_right, handle_up,
};
use crossterm::event::KeyModifiers;

struct Navigation {
    context: Context,
    actions: Vec<Action>,
}

impl Navigation {
    fn swarm(detail: bool) -> Self {
        Self {
            context: Context {
                view_mode: ViewMode::Swarm,
                show_help: false,
                symbols_active: false,
                model_picker_active: false,
                swarm_detail: detail,
                ralph_detail: false,
                bus_filter: false,
                bus_detail: false,
                slash_navigable: false,
                subagent_detail: false,
                subagent_scroll: 0,
                protocol_scroll: 0,
            },
            actions: Vec::new(),
        }
    }
}

impl Host for Navigation {
    fn context(&self) -> Context {
        self.context.clone()
    }
    fn apply(&mut self, action: Action) {
        self.actions.push(action);
    }
    fn file_picker_escape(&mut self) -> bool {
        false
    }
}

#[test]
fn vertical_arrows_select_agents_in_both_swarm_modes() {
    for detail in [false, true] {
        let mut nav = Navigation::swarm(detail);
        handle_down(&mut nav, KeyModifiers::NONE);
        handle_up(&mut nav, KeyModifiers::NONE);
        assert!(matches!(
            nav.actions.as_slice(),
            [Action::SwarmNext, Action::SwarmPrevious]
        ));
    }
}

#[test]
fn horizontal_arrows_open_and_close_details_only_when_needed() {
    let mut nav = Navigation::swarm(false);
    handle_left(&mut nav, KeyModifiers::NONE);
    assert!(nav.actions.is_empty());
    handle_right(&mut nav, KeyModifiers::NONE);
    assert!(matches!(nav.actions.as_slice(), [Action::SwarmEnterDetail]));
    nav.actions.clear();
    nav.context.swarm_detail = true;
    handle_right(&mut nav, KeyModifiers::NONE);
    assert!(nav.actions.is_empty());
    handle_left(&mut nav, KeyModifiers::NONE);
    assert!(matches!(nav.actions.as_slice(), [Action::SwarmExitDetail]));
}

#[test]
fn overlays_prevent_horizontal_swarm_navigation() {
    for detail in [false, true] {
        for help in [false, true] {
            let mut nav = Navigation::swarm(detail);
            nav.context.show_help = help;
            nav.context.symbols_active = !help;
            handle_left(&mut nav, KeyModifiers::NONE);
            handle_right(&mut nav, KeyModifiers::NONE);
            assert!(nav.actions.is_empty());
        }
    }
}

#[test]
fn horizontal_chat_cursor_navigation_is_unchanged() {
    let mut nav = Navigation::swarm(false);
    nav.context.view_mode = ViewMode::Chat;
    handle_left(&mut nav, KeyModifiers::NONE);
    handle_right(&mut nav, KeyModifiers::NONE);
    handle_left(&mut nav, KeyModifiers::CONTROL);
    handle_right(&mut nav, KeyModifiers::CONTROL);
    assert!(matches!(
        nav.actions.as_slice(),
        [
            Action::CursorLeft,
            Action::CursorRight,
            Action::CursorWordLeft,
            Action::CursorWordRight
        ]
    ));
}
