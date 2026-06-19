//! Tests for the goal-setup prompt state machine.

use super::*;

#[test]
fn typing_and_submitting_collects_goals() {
    let (tx, _rx) = oneshot::channel();
    let mut s = GoalPromptState::new("T", "B", tx);
    for c in "ship docs".chars() {
        s.push_char(c);
    }
    assert!(!s.submit_line());
    assert_eq!(s.goals, vec!["ship docs".to_string()]);
    assert!(s.current.is_empty());
}

#[test]
fn blank_line_signals_done() {
    let (tx, _rx) = oneshot::channel();
    let mut s = GoalPromptState::new("T", "B", tx);
    assert!(s.submit_line());
}

#[tokio::test]
async fn finish_delivers_goals() {
    let (tx, rx) = oneshot::channel();
    let mut s = GoalPromptState::new("T", "B", tx);
    s.goals.push("a".into());
    s.finish();
    assert_eq!(rx.await.unwrap(), vec!["a".to_string()]);
}
