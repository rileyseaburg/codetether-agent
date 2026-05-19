use super::*;
use crate::swarm::{SubTask, SwarmConfig};

fn validator() -> SwarmValidator {
    SwarmValidator::new(
        SwarmConfig::default(),
        "test".to_string(),
        "model".to_string(),
    )
}

#[test]
fn test_validate_dependencies_no_cycle() {
    let subtasks = vec![
        SubTask::new("Task A", "Do A"),
        SubTask::new("Task B", "Do B").with_dependencies(vec![]),
        SubTask::new("Task C", "Do C"),
    ];
    let mut issues = Vec::new();
    validator().validate_dependencies(&subtasks, &mut issues);
    assert!(issues.is_empty());
}

#[test]
fn test_validate_dependencies_cycle() {
    let mut a = SubTask::new("Task A", "Do A");
    let mut b = SubTask::new("Task B", "Do B");
    let mut c = SubTask::new("Task C", "Do C");
    a.dependencies = vec![c.id.clone()];
    b.dependencies = vec![a.id.clone()];
    c.dependencies = vec![b.id.clone()];
    let mut issues = Vec::new();
    validator().validate_dependencies(&[a, b, c], &mut issues);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].severity, IssueSeverity::Error);
    assert!(issues[0].message.contains("Circular dependency"));
}

#[test]
fn test_validate_dependencies_missing() {
    let subtask =
        SubTask::new("Task A", "Do A").with_dependencies(vec!["non-existent-id".to_string()]);
    let mut issues = Vec::new();
    validator().validate_dependencies(&[subtask], &mut issues);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].severity, IssueSeverity::Error);
    assert!(issues[0].message.contains("missing subtask"));
}
