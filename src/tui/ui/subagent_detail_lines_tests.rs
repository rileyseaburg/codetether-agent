use super::*;
use crate::provider::{ContentPart, Message, Role};
use crate::tui::app::state::SpawnedAgent;

#[test]
fn managed_detail_keeps_full_tool_input_and_output() {
    let mut state = AppState::default();
    let mut session = futures::executor::block_on(crate::session::Session::new()).unwrap();
    session.messages = vec![
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "1".into(),
                name: "read_file".into(),
                arguments: "{\"path\":\"src/lib.rs\"}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "1".into(),
                content: "complete file output".into(),
            }],
        },
    ];
    state.spawned_agents.insert(
        "observer".into(),
        SpawnedAgent {
            name: "observer".into(),
            instructions: "watch".into(),
            parent: None,
            depth: 0,
            session,
            model_id: Some("openai-codex/test".into()),
            is_processing: true,
        },
    );
    state.active_spawned_agent = Some("observer".into());
    let text = lines(&state)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("Tool call: read_file"));
    assert!(text.contains("src/lib.rs"));
    assert!(text.contains("complete file output"));
}
