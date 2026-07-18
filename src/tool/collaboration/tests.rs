use super::register;
use crate::tool::ToolRegistry;

#[test]
fn registers_first_class_collaboration_tools() {
    let mut registry = ToolRegistry::new();
    register(&mut registry);
    for id in [
        "agent",
        "spawn_agent",
        "followup_task",
        "list_agents",
        "wait_agent",
        "interrupt_agent",
        "close_agent",
        "resume_agent",
        "send_input",
        "send_message",
    ] {
        assert!(registry.contains(id), "missing {id}");
    }
}
