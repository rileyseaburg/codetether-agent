use super::build;
use crate::swarm::executor::AgentLoopExit;

#[test]
fn incomplete_ephemeral_run_is_not_successful() {
    let result = build(
        "once",
        Ok(("partial".into(), 50, 3, AgentLoopExit::MaxStepsReached)),
        None,
    );
    assert!(!result.success);
    assert!(result.output.contains("partial"));
}

#[test]
fn completed_ephemeral_run_returns_output() {
    let result = build(
        "once",
        Ok((
            "evidence\nSTATUS: completed".into(),
            2,
            1,
            AgentLoopExit::Completed,
        )),
        None,
    );
    assert!(result.success);
    assert!(result.output.contains("evidence"));
}
