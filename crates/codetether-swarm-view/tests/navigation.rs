//! Selection, scroll reset and rendered navigation hints.
use codetether_swarm_core::SubTaskStatus;
use codetether_swarm_view::{SubTaskInfo, SwarmViewState, render_swarm_view};
use ratatui::{Terminal, backend::TestBackend};

fn task(id: &str) -> SubTaskInfo {
    SubTaskInfo {
        id: id.into(),
        name: id.into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: vec![],
        agent_name: None,
        current_tool: None,
        steps: 0,
        max_steps: 10,
        tool_call_history: vec![],
        messages: vec![],
        output: Some(format!("Output for {id}")),
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}

fn screen(state: &mut SwarmViewState) -> String {
    let mut terminal = Terminal::new(TestBackend::new(100, 24)).unwrap();
    terminal
        .draw(|frame| render_swarm_view(frame, state, frame.area()))
        .unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

#[test]
fn switching_agents_resets_scroll_and_displays_new_output() {
    let mut state = SwarmViewState::new();
    state.subtasks = vec![task("one"), task("two")];
    state.enter_detail();
    for (next, index, output) in [(true, 1, "two"), (true, 0, "one"), (false, 1, "two")] {
        state.detail_scroll_down(100);
        if next {
            state.select_next();
        } else {
            state.select_prev();
        }
        assert!(state.detail_mode);
        assert_eq!(state.selected_index, index);
        assert_eq!(state.list_state.selected(), Some(index));
        assert_eq!(state.detail_scroll, 0);
        assert!(screen(&mut state).contains(&format!("Output for {output}")));
    }
}

#[test]
fn empty_swarm_navigation_is_safe() {
    let mut state = SwarmViewState::new();
    state.select_prev();
    state.select_next();
    state.enter_detail();
    assert_eq!(state.selected_index, 0);
    assert!(!state.detail_mode);
    assert!(screen(&mut state).contains("SubTasks (none yet)"));
}

#[test]
fn rendered_hints_describe_selection_open_back_and_scroll() {
    let mut state = SwarmViewState::new();
    state.subtasks = vec![task("one")];
    let overview = screen(&mut state);
    assert!(overview.contains("↑↓:select"));
    assert!(overview.contains("Enter/→:detail"));
    state.enter_detail();
    let detail = screen(&mut state);
    assert!(detail.contains("Esc/←: Back"));
    assert!(detail.contains("PgUp/PgDn: Scroll"));
    assert!(detail.contains("↑/↓: Prev/Next agent"));
    state.detail_scroll_down(10);
    state.exit_detail();
    assert!(!state.detail_mode);
    assert_eq!(state.detail_scroll, 0);
}
