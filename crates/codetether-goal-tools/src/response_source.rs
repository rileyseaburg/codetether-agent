//! Surface source evidence only while it still describes the current objective.
use codetether_goal_core::{Goal, TaskEvent};
use serde_json::{Value, json};

pub(super) fn find(events: &[TaskEvent], goal: &Goal) -> Value {
    events
        .iter()
        .rev()
        .find_map(|event| {
            let TaskEvent::GoalSet {
                goal_id,
                objective,
                source_session_id,
                source_turn_id,
                source_text_hash,
                source_kind,
                confidence,
                ..
            } = event
            else {
                return None;
            };
            if goal_id != &goal.id || objective != &goal.objective {
                return None;
            }
            let anchored = !source_turn_id.is_empty() && !source_text_hash.is_empty();
            Some(json!({
                "sessionId": source_session_id, "turnId": source_turn_id,
                "textHash": source_text_hash, "kind": source_kind, "confidence": confidence,
                "hasSourceReference": anchored,
                "guidance": if anchored {
                    "Compare the objective with this user message; derived wording is not additional authorization."
                } else {
                    "No canonical source was recorded. Treat the goal wording as unverified and consult the original user request."
                },
            }))
        })
        .unwrap_or(Value::Null)
}
