use crate::tool::ToolResult;

#[test]
fn remote_turn_has_a_parent_scoped_dashboard_transcript() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("remote-{suffix}");
    let parent = format!("parent-{suffix}");
    let turn = crate::tool::agent::message::remote::observation::begin(
        &name,
        Some(&parent),
        "inspect run 2377",
    );

    let peers = super::list_agent_tool_agents_for_parent(&parent);
    let peer = peers.iter().find(|peer| peer.name == name).unwrap();
    assert!(peer.is_remote);
    assert!(peer.is_processing);
    let transcript = super::agent_tool_transcript_for_parent(&name, &parent).unwrap();
    assert_eq!(transcript.len(), 1);

    turn.complete(&ToolResult::success("lint failed"));
    let done = super::find_agent_tool_agent_for_parent(&name, &parent).unwrap();
    assert!(!done.is_processing);
}
