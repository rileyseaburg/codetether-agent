use super::model_selection::ModelSelection;
use super::task_result::{TaskResult, completed};
use crate::swarm::executor::AgentLoopExit;

#[test]
fn only_completed_agent_loops_are_successful() {
    assert!(completed(AgentLoopExit::Completed, "STATUS: completed"));
    assert!(!completed(AgentLoopExit::Completed, "STATUS: blocked"));
    assert!(!completed(AgentLoopExit::MaxStepsReached, ""));
    assert!(!completed(AgentLoopExit::TimedOut, ""));
}

#[test]
fn failed_result_preserves_model_diagnostics() {
    let model = ModelSelection {
        requested_model: Some("provider/model:high".into()),
        resolved_provider: "provider".into(),
        resolved_model: "model:high".into(),
    };
    let result = TaskResult::failed("task-1".into(), "inspect".into(), "boom".into(), model);

    assert_eq!(result.task_id, "task-1");
    assert_eq!(result.error.as_deref(), Some("boom"));
    assert_eq!(
        result.model.requested_model.as_deref(),
        Some("provider/model:high")
    );
    assert_eq!(result.model.resolved_model, "model:high");
    assert!(!result.success);

    let json = serde_json::to_value(result).expect("result should serialize");
    assert_eq!(json["model"]["requested_model"], "provider/model:high");
    assert_eq!(json["model"]["resolved_provider"], "provider");
    assert_eq!(json["model"]["resolved_model"], "model:high");
}
