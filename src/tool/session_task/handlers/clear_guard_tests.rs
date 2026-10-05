//! Model-facing `clear_goal` cannot discard an active, unverified goal.

use crate::approval::test_env::lock_env;
use crate::session::tasks::TaskLog;
use serde_json::json;

#[tokio::test]
async fn active_goal_is_not_cleared() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let log = TaskLog::for_session("clear-guard").unwrap();
    let set = json!({"action": "set_goal", "objective": "Ship"});
    super::super::dispatch::run(&log, serde_json::from_value(set).unwrap())
        .await
        .unwrap();
    let clear = json!({"action": "clear_goal", "reason": "done"});
    let result = super::super::dispatch::run(&log, serde_json::from_value(clear).unwrap())
        .await
        .unwrap();
    assert!(!result.success, "{}", result.output);
    let state = crate::session::tasks::TaskState::from_log(&log.read_all().await.unwrap());
    assert!(state.goal.is_some());
    crate::session::tasks::runtime::set_status(
        "clear-guard",
        crate::session::tasks::GoalStatus::Paused,
    )
    .await
    .unwrap();
    let clear = json!({"action": "clear_goal"});
    let paused = super::super::dispatch::run(&log, serde_json::from_value(clear).unwrap())
        .await
        .unwrap();
    assert!(
        !paused.success,
        "paused goal must not be cleared by the model"
    );
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
