//! A real session must keep approval corrections after eviction and reload.
use super::*;

#[tokio::test]
async fn approval_feedback_survives_cold_reload_as_pinned_human_source() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("feedback.json");
    let mut session = Session::new().await.unwrap();
    let feedback = "Use the current local server and the existing Playwright example.";
    append(&mut session, "denied-call", feedback.into());
    for _ in 0..8 {
        session.add_message(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Later tool investigation".into(),
            }],
        });
    }
    crate::session::store::save(&session, &path).await.unwrap();
    let loaded = crate::session::store::load(&path, 2).await.unwrap().session;
    let constraints = crate::session::store::constraints::prefix(&loaded);
    let source = constraints
        .iter()
        .find(|source| source.source_turn == 0)
        .unwrap();
    assert_eq!(source.source_role, Role::User);
    assert!(source.text.contains(feedback));
    assert!(source.text.contains("denied-call"));
    assert!(source.has_valid_hash());
}
