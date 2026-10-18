use super::*;
use codetether_provider_core::ToolDefinition;

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
fn misleading_readonly_metadata_cannot_enable_mutating_tools() {
    let task = crate::SubTask::new("Edit Code", "Edit the auth module")
        .with_specialty("Researcher")
        .with_needs_worktree(false);

    assert!(!task.is_read_only());
    assert!(task.needs_worktree());
}

#[test]
fn honest_readonly_metadata_keeps_readonly_tools() {
    let task = crate::SubTask::new("Research", "Find unsafe usages")
        .with_specialty("Researcher")
        .with_needs_worktree(false);

    assert!(task.is_read_only());
    assert!(!task.needs_worktree());
}
