use super::*;
use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;

#[test]
fn read_only_definitions_hide_mutating_tools() {
    let tools = vec![
        ToolDefinition {
            name: "read".into(),
            description: "".into(),
            parameters: serde_json::json!({}),
        },
        ToolDefinition {
            name: "bash".into(),
            description: "".into(),
            parameters: serde_json::json!({}),
        },
    ];
    let names: Vec<String> = definitions(&tools, true)
        .into_iter()
        .map(|tool| tool.name)
        .collect();
    assert_eq!(names, vec!["read"]);
}

#[test]
fn read_only_registry_removes_mutating_tools() {
    let mut registry = ToolRegistry::with_defaults();
    restrict_registry(&mut registry, true);
    for denied in ["write", "edit", "multiedit", "patch", "bash", "ralph", "go"] {
        assert!(!registry.contains(denied), "{denied} should be blocked");
    }
    assert!(registry.contains("read"));
}

#[test]
fn misleading_readonly_metadata_cannot_enable_mutating_tools() {
    let task = crate::swarm::SubTask::new("Edit Code", "Edit the auth module")
        .with_specialty("Researcher")
        .with_needs_worktree(false);

    assert!(!task.is_read_only());
    assert!(task.needs_worktree());
}

#[test]
fn honest_readonly_metadata_keeps_readonly_tools() {
    let task = crate::swarm::SubTask::new("Research", "Find unsafe usages")
        .with_specialty("Researcher")
        .with_needs_worktree(false);

    assert!(task.is_read_only());
    assert!(!task.needs_worktree());
}
