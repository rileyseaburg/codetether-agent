//! Tests for the unified agent dashboard.

use super::*;
use crate::swarm::SubTaskStatus;
use crate::tool::agent::bridge::AgentSnapshot;
use crate::tui::app::state::AppState;

#[path = "subagent_lines_test_fixture.rs"]
mod fixture;

#[test]
fn empty_state_mentions_all_agent_entry_points() {
    let text = fixture::text(&compose(&AppState::default(), &[]));
    assert!(text.contains("No agents yet"));
    assert!(text.contains("/swarm <task>"));
}

#[test]
fn swarm_only_state_appears_in_agents_dashboard() {
    let mut state = AppState::default();
    state.swarm.task = "audit repository".into();
    state.swarm.subtasks = vec![fixture::task("review", SubTaskStatus::Running)];
    let text = fixture::text(&compose(&state, &[]));
    assert!(text.contains("swarm: 1"));
    assert!(text.contains("worker-review"));
    assert!(text.contains("audit repository"));
    assert!(text.contains("grep"));
}

#[test]
fn tool_agents_and_swarm_workers_share_the_dashboard() {
    let mut state = AppState::default();
    state.swarm.subtasks = vec![fixture::task("test", SubTaskStatus::Pending)];
    let tool = AgentSnapshot {
        name: "planner".into(),
        instructions: "Plan work".into(),
        message_count: 3,
        model_id: None,
        parent: None,
        depth: 0,
        is_processing: true,
    };
    let text = fixture::text(&compose(&state, &[tool]));
    assert!(text.contains("managed: 1 · swarm: 1"));
    assert!(text.contains("planner"));
    assert!(text.contains("[working]"));
    assert!(text.contains("worker-test"));
}
