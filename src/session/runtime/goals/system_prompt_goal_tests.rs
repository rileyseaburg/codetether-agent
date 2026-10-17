//! Prompt rendering retains the full existing goal without mutating its log.

use super::from_log;
use crate::session::tasks::{GoalSourceKind, TaskEvent, TaskLog, TaskState, turn_instructions};
use chrono::Utc;

#[test]
fn turn_contract_preserves_existing_goal_criteria_and_budget() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.tasks.jsonl"));
    let event = TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: "original-goal".into(),
        objective: "Deliver the entire requested outcome".into(),
        success_criteria: vec!["Normal agents".into(), "Delegated agents".into()],
        forbidden: Vec::new(),
        source_session_id: String::new(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    };
    log.append_blocking(&event).unwrap();
    let before = std::fs::read(log.path()).unwrap();
    let base = turn_instructions::append("Keep project rules".into());
    let prompt = from_log(base, &log);
    assert!(prompt.starts_with("Keep project rules"));
    assert!(prompt.contains("Deliver the entire requested outcome"));
    assert!(prompt.contains("Normal agents"));
    assert!(prompt.contains("Delegated agents"));
    assert!(prompt.contains("Before ending every turn"));
    assert_eq!(std::fs::read(log.path()).unwrap(), before);
    let state = TaskState::from_log(&log.read_all_blocking().unwrap());
    let goal = state.goal.unwrap();
    assert_eq!(goal.id, "original-goal");
    assert_eq!(goal.token_budget, None);
}
