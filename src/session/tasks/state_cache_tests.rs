use super::load;
use crate::session::tasks::{GoalSourceKind, TaskEvent, TaskLog};
use chrono::Utc;

fn goal(objective: &str) -> TaskEvent {
    TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: objective.into(),
        objective: objective.into(),
        success_criteria: Vec::new(),
        forbidden: Vec::new(),
        source_session_id: "s1".into(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    }
}

#[tokio::test]
async fn cache_reflects_appends_and_missing_logs() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("s.tasks.jsonl"));
    assert!(load(&log).unwrap().goal.is_none());
    log.append(&goal("first")).await.unwrap();
    assert_eq!(load(&log).unwrap().goal.unwrap().objective, "first");
    assert_eq!(load(&log).unwrap().goal.unwrap().objective, "first");
    log.append(&goal("second objective")).await.unwrap();
    let state = load(&log).unwrap();
    assert_eq!(state.goal.unwrap().objective, "second objective");
}
