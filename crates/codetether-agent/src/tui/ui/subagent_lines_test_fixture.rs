//! Fixtures for unified agent-dashboard tests.

use ratatui::text::Line;

use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SubTaskInfo;

pub fn task(name: &str, status: SubTaskStatus) -> SubTaskInfo {
    SubTaskInfo {
        id: format!("id-{name}"),
        name: name.to_string(),
        status,
        stage: 1,
        dependencies: vec![],
        agent_name: Some(format!("worker-{name}")),
        current_tool: Some("grep".to_string()),
        steps: 2,
        max_steps: 10,
        tool_call_history: vec![],
        messages: vec![],
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}

pub fn text(lines: &[Line<'static>]) -> String {
    lines
        .iter()
        .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
        .collect::<Vec<_>>()
        .join("\n")
}
