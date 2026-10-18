use codetether_agent::swarm::k8s_result::final_result;
use codetether_agent::swarm::kubernetes_executor::SWARM_SUBTASK_RESULT_PREFIX;
use codetether_agent::swarm::subtask::SubTaskResult;

#[test]
fn reports_missing_runner_support() {
    let result = final_result("task-1", 100, Some(127), Some("Error"), "not found");
    assert!(!result.success);
    assert_eq!(result.result, "");
    assert_eq!(result.artifacts, Vec::<String>::new());
    assert!(result.error.unwrap().contains("runner missing"));
}

#[test]
fn reports_unstructured_child_failure() {
    let result = final_result("task-2", 200, Some(2), Some("Failed"), "logs only");
    assert!(!result.success);
    assert_eq!(result.result, "");
    assert!(result.error.unwrap().contains("did not emit structured"));
}

#[test]
fn parses_structured_success() {
    let logs = structured_success_logs();
    let result = final_result("task-3", 300, Some(0), None, &logs);
    assert!(result.success);
    assert_eq!(result.result, "typed summary");
    assert_eq!(result.artifacts, vec!["artifact.txt".to_string()]);
    assert_eq!(result.execution_time_ms, 300);
}

fn structured_success_logs() -> String {
    let json = serde_json::to_string(&structured_success()).unwrap();
    format!("noise\n{SWARM_SUBTASK_RESULT_PREFIX}{json}\n")
}

fn structured_success() -> SubTaskResult {
    SubTaskResult {
        subtask_id: "task-3".into(),
        subagent_id: "agent-task-3".into(),
        success: true,
        result: "typed summary".into(),
        steps: 4,
        tool_calls: 1,
        execution_time_ms: 0,
        error: None,
        artifacts: vec!["artifact.txt".into()],
        retry_count: 0,
    }
}
