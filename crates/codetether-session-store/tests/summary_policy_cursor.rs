mod constraint_support;
use codetether_context_policy::summary_index::projection_name;
use codetether_session_store::{projection, save};
use constraint_support::Session;
mod summary_cursor_support;
use summary_cursor_support::next;
#[tokio::test]
async fn new_summary_policy_replays_source_without_destroying_old_evidence() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    let mut session = Session::default();
    for _ in 0..24 {
        session.append("original source");
    }
    save(&session, &path).await.unwrap();
    let old = next(&path, "rlm").unwrap();
    projection::commit(
        old.ticket,
        serde_json::json!({"documents":[{"start":0,"end":24,"note":"old"}]}),
    )
    .await
    .unwrap();
    assert!(next(&path, "rlm").is_none());
    let name = projection_name();
    let current = next(&path, &name).unwrap();
    assert_eq!((current.ticket.from, current.ticket.to), (0, 24));
    projection::commit(
        current.ticket,
        serde_json::json!({"documents":[{"start":0,"end":24,"note":"new"}]}),
    )
    .await
    .unwrap();
    assert!(next(&path, &name).is_none());
    for (name, note) in [("rlm", "old"), (name.as_str(), "new")] {
        let view = projection::read(&path, "constraints-session", name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(view["documents"][0]["note"], note);
    }
    session.messages[17] = constraint_support::message("edited source");
    save(&session, &path).await.unwrap();
    assert_eq!(next(&path, &name).unwrap().ticket.from, 16);
}
