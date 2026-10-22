//! A checkpoint sees one SQLite snapshot and rejects missing canonical sources.
use super::super::save;
use crate::session::Session;
#[path = "checkpoint_writer.rs"]
mod writer;
#[tokio::test]
async fn checkpoint_snapshot_is_consistent_and_gaps_are_errors() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("snapshot.json");
    let mut session = Session::new().await.unwrap();
    session
        .messages
        .extend((0..128).map(|_| super::message("before")));
    save(&session, &path).await.unwrap();
    let db_path = root.path().join("sessions.sqlite3");
    let writer_path = db_path.clone();
    let id = session.id.clone();
    let (seen, total) = codetether_session_store::fold_messages::<Session, _, _>(
        &path,
        &session.id,
        Vec::new(),
        move |seen, index, message| {
            if index == 0 {
                writer::replace(&writer_path, &id)?;
            }
            seen.push(message);
            Ok(())
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 128);
    assert_eq!(
        serde_json::to_value(&seen[100]).unwrap(),
        serde_json::to_value(super::message("before")).unwrap()
    );
    let current = session
        .build_run_checkpoint("repair", 2, root.path())
        .await
        .unwrap();
    assert_eq!(current.user_context[100].text, "after");
    writer::remove(&db_path, &session.id).unwrap();
    let error = session
        .build_run_checkpoint("repair", 2, root.path())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("canonical message gap"));
}
