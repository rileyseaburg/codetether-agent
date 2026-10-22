use super::fixture::Session;
use crate::{PinSource, correct};
use codetether_context_policy::{pages::PageKind, state_header::ConstraintRecord};
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::{Value, json};

fn fixture() -> (Session, Value) {
    let mut session = Session::new();
    session.messages[0].content = vec![ContentPart::Text {
        text: "Keep evidence. Use 🦀 red. Do not deploy.".into(),
    }];
    let hash = session.constraint_source(100).unwrap().text_hash;
    (
        session,
        json!({"replacement_index":102,"source_text":"🦀 red","source_hash":hash}),
    )
}

#[tokio::test]
async fn partial_correction_is_source_backed_and_idempotent() {
    let (mut session, args) = fixture();
    correct::apply(&mut session, 100, &args).await.unwrap();
    assert_eq!(session.pages[0], PageKind::Constraint);
    assert_eq!(session.pages[2], PageKind::Constraint);
    assert_eq!(session.corrections.len(), 1);
    let edit = &session.corrections[0];
    assert_eq!(
        &session.constraint_source(100).unwrap().text[edit.start..edit.end],
        "🦀 red"
    );
    correct::apply(&mut session, 100, &args).await.unwrap();
    assert_eq!(session.saves, 1);
    session.pages[2] = PageKind::Conversation;
    assert!(correct::apply(&mut session, 100, &args).await.is_err());
}

#[tokio::test]
async fn ambiguous_stale_missing_and_non_user_sources_leave_no_edits() {
    for case in 0..6 {
        let (mut session, mut args) = fixture();
        match case {
            0 => args["source_hash"] = json!("stale"),
            1 => args["source_text"] = json!("missing"),
            2 => args["source_text"] = json!(""),
            3 => session.messages[2].role = Role::Assistant,
            4 => args["replacement_index"] = json!(99),
            5 => {
                session.messages[0].content = vec![ContentPart::Text { text: "aaa".into() }];
                args["source_hash"] = json!(session.constraint_source(100).unwrap().text_hash);
                args["source_text"] = json!("aa");
            }
            _ => unreachable!(),
        }
        assert!(
            correct::apply(&mut session, 100, &args).await.is_err(),
            "case {case}"
        );
        assert!(session.corrections.is_empty());
        assert_eq!(session.saves, 0);
        assert_eq!(session.pages[2], PageKind::Conversation);
    }
}

#[tokio::test]
async fn overlapping_corrections_and_failed_saves_do_not_change_pins() {
    let (mut session, mut args) = fixture();
    session.fail_save = true;
    assert!(correct::apply(&mut session, 100, &args).await.is_err());
    assert!(session.corrections.is_empty());
    assert_eq!(session.pages[2], PageKind::Conversation);
    session.fail_save = false;
    correct::apply(&mut session, 100, &args).await.unwrap();
    args["source_text"] = json!("Use 🦀 red.");
    assert!(correct::apply(&mut session, 100, &args).await.is_err());
    assert_eq!(session.corrections.len(), 1);
}

#[tokio::test]
async fn an_evicted_original_can_be_corrected_without_loading_history() {
    let (mut session, mut args) = fixture();
    let source = ConstraintRecord::from_message(
        3,
        &Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "old rule; preserve this".into(),
            }],
        },
        PageKind::Constraint,
    )
    .unwrap();
    args["source_hash"] = json!(source.text_hash);
    args["source_text"] = json!("old rule");
    session.prefix.push(source);
    correct::apply(&mut session, 3, &args).await.unwrap();
    assert_eq!(session.corrections[0].source_turn, 3);
    assert_eq!(session.pages.len(), 3);
}

#[tokio::test]
async fn correct_action_runs_through_the_registered_tool_contract() {
    use codetether_tool_core::Tool;
    let hash = Session::new().constraint_source(100).unwrap().text_hash;
    let result = crate::ContextPinTool::<Session>::new()
        .execute(json!({
            "__ct_session_id":"runtime-correction", "action":"correct", "turn_index":100,
            "replacement_index":102, "source_text":"source", "source_hash":hash,
        }))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert!(
        result
            .output
            .contains("all other source text remains active")
    );
}
