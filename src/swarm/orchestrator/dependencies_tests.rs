use super::resolve;
use std::collections::HashMap;

#[test]
fn resolves_exact_task_names() {
    let names = HashMap::from([("Inspect".to_string(), "task-1".to_string())]);
    assert_eq!(
        resolve(&["Inspect".to_string()], &names).unwrap(),
        vec!["task-1"]
    );
}

#[test]
fn rejects_unknown_dependency_names() {
    let error = resolve(&["invented-id".to_string()], &HashMap::new()).unwrap_err();
    assert!(error.to_string().contains("invented-id"));
}
