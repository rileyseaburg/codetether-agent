//! A verifier commit tolerates usage accounting but not substantive goal changes.
use super::set_status_if_current;
use crate::Journal;
use chrono::Utc;
use codetether_goal_core::{
    GoalEdit, GoalEdited, GoalRuntimeUpdate, GoalStatus, TaskEvent, TaskLog, TaskState,
};
use serde_json::json;

struct Journals;
impl Journal for Journals {
    fn for_session(id: &str) -> anyhow::Result<TaskLog> {
        Ok(TaskLog::at(id))
    }
}

fn declared(path: &str) -> codetether_goal_core::Goal {
    let log = TaskLog::at(path);
    log.append_blocking(
        &serde_json::from_value(json!({
            "kind":"goal_set", "at":Utc::now(), "goal_id":"g", "objective":"Ship it",
            "success_criteria":[], "forbidden":[]
        }))
        .unwrap(),
    )
    .unwrap();
    state(path).goal.unwrap()
}

fn usage(path: &str) {
    TaskLog::at(path)
        .append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
            at: Utc::now(),
            goal_id: "g".into(),
            objective: None,
            status: None,
            token_budget: None,
            token_delta: 500,
            elapsed_seconds: 4,
            continuation_delta: 1,
        }))
        .unwrap();
}

fn state(path: &str) -> TaskState {
    TaskState::from_log(&TaskLog::at(path).read_all_blocking().unwrap())
}

#[tokio::test]
async fn verifier_commit_survives_accounting_during_verification() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("goal.jsonl");
    let path = path.to_str().unwrap();
    let expected = declared(path);
    usage(path);
    usage(path);
    assert!(
        set_status_if_current::<Journals>(path, &expected, GoalStatus::Complete)
            .await
            .unwrap()
    );
    let goal = state(path).goal.unwrap();
    assert_eq!(goal.status, GoalStatus::Complete);
    assert_eq!(goal.tokens_used, 1000);
}

#[tokio::test]
async fn verifier_commit_rejected_after_objective_edit() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("goal.jsonl");
    let path = path.to_str().unwrap();
    let expected = declared(path);
    let request: GoalEdit = serde_json::from_value(json!({
        "goalId":"g", "updatedAt":expected.last_updated_at, "action":"override",
        "objective":"Something else"
    }))
    .unwrap();
    TaskLog::at(path)
        .append_blocking(&TaskEvent::GoalEdited(GoalEdited {
            at: Utc::now(),
            request,
        }))
        .unwrap();
    usage(path);
    assert!(
        !set_status_if_current::<Journals>(path, &expected, GoalStatus::Complete)
            .await
            .unwrap()
    );
    assert_eq!(state(path).goal.unwrap().status, GoalStatus::Active);
}
