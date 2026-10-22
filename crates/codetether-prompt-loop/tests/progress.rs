mod progress_support;
use codetether_prompt_loop::response::progress::track;
use codetether_provider_core::{ContentPart, Role};
use progress_support::*;
#[test]
fn guidance_depends_on_results_and_survives_turn_state_reset() {
    let mut state = State::default();
    for i in 0..20 {
        result(&mut state, i, serde_json::json!({"command":"echo same"}));
        let mut turn = codetether_prompt_loop::LoopState::new(Some(1));
        turn.reset_goal_turn();
    }
    track(&mut state, 20);
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.ledger.stagnant_results, 0);
}
#[test]
fn varied_inspections_still_require_periodic_task_review() {
    let mut state = State::default();
    for i in 0..40 {
        result(
            &mut state,
            i,
            serde_json::json!({"command":format!("inspect file {i}")}),
        );
        track(&mut state, i);
    }
    assert_eq!(state.messages.len(), 2);
    for message in &state.messages {
        assert_eq!(message.role, Role::Developer);
        let ContentPart::Text { text } = &message.content[0] else {
            panic!("review must be text");
        };
        assert!(text.contains("TASK CONTINUITY REVIEW"));
        assert!(text.contains("Read-only investigation is valid"));
    }
}
