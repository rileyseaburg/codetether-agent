use super::detached_message;
use crate::tool::agent::{
    collaboration_runtime::fork_turns::ForkTurns, spawn_request::SpawnRequest,
};

#[test]
fn detached_spawn_returns_the_canonical_agent_id() {
    let request = SpawnRequest {
        name: "reviewer",
        instructions: "review",
        model: "test/model",
        ephemeral: false,
        detach: true,
        parent_workspace: None,
        parent_session_id: Some("parent"),
        fork_turns: ForkTurns::None,
    };
    let output: serde_json::Value =
        serde_json::from_str(&detached_message(&request, "child-id", None)).unwrap();
    assert_eq!(output["agent_id"], "child-id");
    assert_eq!(output["nickname"], "reviewer");
}
