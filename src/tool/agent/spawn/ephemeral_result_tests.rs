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
    for output in ["evidence\nSTATUS: completed", "evidence: reviewed the diff"] {
        let result = build(
            "once",
            Ok((output.into(), 2, 1, AgentLoopExit::Completed)),
            None,
        );
        assert!(result.success);
        assert!(result.output.contains("evidence"));
    }
}

#[test]
fn completed_loop_with_explicit_non_completion_is_not_successful() {
    for output in ["STATUS: blocked", "STATUS: pending"] {
        let result = build(
            "once",
            Ok((output.into(), 2, 1, AgentLoopExit::Completed)),
            None,
        );
        assert!(!result.success);
    }
}
