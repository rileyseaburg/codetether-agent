#[test]
fn refresh_removes_stale_goals_and_recovered_error_snapshots() {
    use codetether_goal_core::TaskLog;
    use codetether_provider_core::runtime_context::{append, split};
    let dir = tempfile::tempdir().unwrap();
    let empty = TaskLog::at(dir.path().join("empty.jsonl"));
    let old = append("stable rules", "old goal");
    assert_eq!(
        codetether_goal_runtime::compose_from_log(old.clone(), &empty),
        "stable rules"
    );
    let corrupt = TaskLog::at(dir.path().join("corrupt.jsonl"));
    std::fs::write(corrupt.path(), "invalid event\n").unwrap();
    let unavailable = codetether_goal_runtime::compose_from_log(old, &corrupt);
    let (stable, state) = split(&unavailable);
    assert_eq!(stable, "stable rules");
    assert!(state.unwrap().contains("GOAL_GOVERNANCE_UNAVAILABLE"));
    assert_eq!(
        codetether_goal_runtime::compose_from_log(unavailable, &empty),
        "stable rules"
    );
}

struct FileJournal;
impl codetether_goal_runtime::Journal for FileJournal {
    fn for_session(path: &str) -> anyhow::Result<codetether_goal_core::TaskLog> {
        Ok(codetether_goal_core::TaskLog::at(std::path::PathBuf::from(
            path,
        )))
    }
}

#[test]
fn repeated_native_composition_refreshes_accounting_without_growing_instructions() {
    use chrono::Utc;
    use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, TaskEvent, TaskLog};
    use codetether_provider_core::runtime_context::split;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("journal.jsonl");
    let log = TaskLog::at(path.clone());
    log.append_blocking(&TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: "Finish".into(),
        success_criteria: vec![],
        forbidden: vec!["Do not deploy".into()],
        source_session_id: "session".into(),
        source_turn_id: "0".into(),
        source_text_hash: "hash".into(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })
    .unwrap();
    let first =
        codetether_goal_runtime::compose::<FileJournal>("stable rules", path.to_str().unwrap());
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: None,
        status: None,
        token_budget: None,
        token_delta: 750,
        elapsed_seconds: 4,
        continuation_delta: 1,
    }))
    .unwrap();
    let second = codetether_goal_runtime::compose::<FileJournal>(&first, path.to_str().unwrap());
    let (stable_a, state_a) = split(&first);
    let (stable_b, state_b) = split(&second);
    assert_eq!(stable_a, stable_b);
    assert_ne!(state_a, state_b);
    assert!(state_b.unwrap().contains("750 tokens"));
    assert_eq!(second.matches("<codetether_runtime_goal_state>").count(), 1);
}
