mod corrections;

#[tokio::test]
async fn rejected_persistence_restores_both_supersession_pages() {
    let mut session = Session::new();
    session.fail_save = true;
    let before = session.pages.clone();
    assert!(
        apply_pin(&mut session, 100, "supersede", Some(102))
            .await
            .is_err()
    );
    assert_eq!(session.pages, before);
}
mod fixture;
use crate::logic::apply_pin;
use codetether_context_policy::pages::PageKind;
use fixture::Session;

#[tokio::test]
async fn supersession_is_atomic_absolute_and_retryable() {
    let mut session = Session::new();
    apply_pin(&mut session, 100, "supersede", Some(102))
        .await
        .unwrap();
    assert_eq!(
        session.pages[0],
        PageKind::SupersededConstraint { by_turn: 102 }
    );
    assert_eq!(session.pages[2], PageKind::Constraint);
    assert_eq!(session.saves, 1);
    apply_pin(&mut session, 100, "supersede", Some(102))
        .await
        .unwrap();
    assert_eq!(session.saves, 2);
    apply_pin(&mut session, 102, "unpin", None).await.unwrap();
    assert_eq!(session.pages[2], PageKind::Conversation);
    assert!(
        apply_pin(&mut session, 100, "supersede", Some(102))
            .await
            .is_err()
    );
    assert_eq!(session.pages[2], PageKind::Conversation);
    assert_eq!(session.saves, 3);
    apply_pin(&mut session, 100, "unpin", None).await.unwrap();
    assert_eq!(
        session.pages[0],
        PageKind::SupersededConstraint { by_turn: 102 }
    );
}

#[tokio::test]
async fn invalid_replacement_or_old_index_cannot_mutate_either_page() {
    for (old, replacement) in [
        (100, Some(101)),
        (100, Some(100)),
        (0, Some(102)),
        (100, None),
    ] {
        let mut session = Session::new();
        let before = session.pages.clone();
        assert!(
            apply_pin(&mut session, old, "supersede", replacement)
                .await
                .is_err()
        );
        assert_eq!(session.pages, before);
        assert_eq!(session.saves, 0);
    }
}

#[tokio::test]
async fn absolute_pin_updates_the_intended_loaded_source() {
    let mut session = Session::new();
    apply_pin(&mut session, 102, "pin", None).await.unwrap();
    assert_eq!(
        session.pages,
        [
            PageKind::Constraint,
            PageKind::Conversation,
            PageKind::Constraint
        ]
    );
}

#[tokio::test]
async fn tool_runtime_resolves_the_calling_session_and_persists_supersession() {
    use codetether_tool_core::Tool;
    let result = crate::ContextPinTool::<Session>::new()
        .execute(serde_json::json!({
            "__ct_session_id": "runtime-session", "action": "supersede",
            "turn_index": 100, "replacement_index": 102,
        }))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert!(result.output.contains("superseded"));
    assert_eq!(
        fixture::saved()[0],
        PageKind::SupersededConstraint { by_turn: 102 }
    );
    assert_eq!(fixture::saved()[2], PageKind::Constraint);
}
