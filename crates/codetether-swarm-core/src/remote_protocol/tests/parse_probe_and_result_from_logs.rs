use super::*;

#[test]
fn parse_probe_and_result_from_logs() {
    let probe = RemoteBranchProbe {
        subtask_id: "subtask-1".to_string(),
        compile_ok: true,
        changed_files: vec!["src/main.rs".to_string()],
        changed_lines: 42,
    };
    let result = SubTaskResult {
        subtask_id: "subtask-1".to_string(),
        subagent_id: "agent-subtask-1".to_string(),
        success: true,
        result: "ok".to_string(),
        steps: 3,
        tool_calls: 2,
        execution_time_ms: 1000,
        error: None,
        artifacts: Vec::new(),
        retry_count: 0,
    };
    let logs = format!(
        "line\n{}{}\nline\n{}{}\n",
        SWARM_SUBTASK_PROBE_PREFIX,
        serde_json::to_string(&probe).expect("probe json"),
        SWARM_SUBTASK_RESULT_PREFIX,
        serde_json::to_string(&result).expect("result json"),
    );
    let parsed_probe = latest_probe_from_logs(&logs).expect("probe");
    let parsed_result = result_from_logs(&logs).expect("result");
    assert!(parsed_probe.compile_ok);
    assert_eq!(parsed_probe.changed_lines, 42);
    assert_eq!(parsed_result.subtask_id, "subtask-1");
    assert!(parsed_result.success);
}
