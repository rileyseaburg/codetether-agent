//! Goal instruction injection for sessions with no persisted goal.

use super::compose;
use crate::session::tasks::TaskLog;

#[test]
fn goal_less_session_gets_turn_instructions_without_creating_a_goal() {
    let id = format!("turn-goal-{}", uuid::Uuid::new_v4());
    let log = TaskLog::for_session(&id).unwrap();
    assert!(!log.path().exists());
    let prompt = compose("Normal agent instructions", &id);
    assert!(prompt.starts_with("Normal agent instructions"));
    assert!(prompt.contains("Before ending every turn"));
    assert!(prompt.contains("inspect the objective with `get_goal`"));
    assert!(!prompt.contains("`session_task` action `set_goal`"));
    assert!(
        !log.path().exists(),
        "prompt rendering must not create a goal"
    );
    assert_eq!(compose(&prompt, &id), prompt);
}
