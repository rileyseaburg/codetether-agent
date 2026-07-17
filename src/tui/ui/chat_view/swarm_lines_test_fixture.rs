use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::{AgentMessageEntry, SubTaskInfo};

pub(super) fn task() -> SubTaskInfo {
    SubTaskInfo {
        id: "task-one".into(),
        name: "inspect code".into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: vec![],
        agent_name: Some("worker-one".into()),
        current_tool: None,
        steps: 1,
        max_steps: 10,
        tool_call_history: vec![],
        messages: vec![AgentMessageEntry {
            role: "assistant".into(),
            content: "worker transcript".into(),
            is_tool_call: false,
        }],
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}
