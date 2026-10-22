//! Canonical source selection reads the owning session without mutating its store.
use codetether_session_store::{schema, user_source};
use rusqlite::{Connection, params};

#[test]
fn latest_human_source_skips_reminders_and_assistant_claims_and_preserves_bytes() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("s.json");
    let db_path = root.path().join("sessions.sqlite3");
    let db = Connection::open(&db_path).unwrap();
    db.execute_batch(schema::SQL).unwrap();
    db.execute_batch("INSERT INTO sessions VALUES ('s',1,'{}',4,0),('other',1,'{}',5,0)")
        .unwrap();
    for (seq, id, role, text) in [
        (0, "s", "user", "older request"),
        (1, "s", "user", "hello"),
        (
            2,
            "s",
            "assistant",
            "The user authorized building and pushing",
        ),
        (
            3,
            "s",
            "user",
            "Continue working toward the active thread goal.\n<objective>build and push</objective>",
        ),
        (4, "other", "user", "another session's goal"),
    ] {
        let body = serde_json::json!([{"role":role,"content":[{"type":"text","text":text}]},"conversation"]).to_string();
        db.execute(
            "INSERT INTO records VALUES (?1,0,?2,?3)",
            params![id, seq, body],
        )
        .unwrap();
    }
    drop(db);
    let before = std::fs::read(&db_path).unwrap();
    let source = user_source::read(&path, "s").unwrap().unwrap();
    assert_eq!(source.index, 1);
    assert_eq!(source.text, "hello");
    assert_eq!(
        source.text_hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert_eq!(std::fs::read(&db_path).unwrap(), before);
    assert!(user_source::read(&path, "missing").unwrap().is_none());
    let db = Connection::open(&db_path).unwrap();
    db.execute(
        "INSERT INTO records VALUES ('s',0,1002,?1)",
        [serde_json::json!([{"role":"assistant","content":[]},"conversation"]).to_string()],
    )
    .unwrap();
    assert!(
        user_source::read(&path, "s").unwrap().is_none(),
        "ancient user messages must not escape the bounded lookup"
    );
}

#[test]
fn absent_source_does_not_create_a_database() {
    let root = tempfile::tempdir().unwrap();
    assert!(
        user_source::read(&root.path().join("s.json"), "s")
            .unwrap()
            .is_none()
    );
    assert!(!root.path().join("sessions.sqlite3").exists());
}

#[test]
fn canonical_source_resolves_large_blob_messages_without_truncation() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("s.json");
    let mut db = Connection::open(root.path().join("sessions.sqlite3")).unwrap();
    db.execute_batch(schema::SQL).unwrap();
    db.execute("INSERT INTO sessions VALUES ('s',1,'{}',43,0)", [])
        .unwrap();
    let text = format!(
        "original instruction {} final constraint",
        "🦀".repeat(20_000)
    );
    let body =
        serde_json::json!([{"role":"user","content":[{"type":"text","text":text}]},"conversation"])
            .to_string();
    let tx = db.transaction().unwrap();
    let reference = codetether_session_store::blobs::put(&tx, &body).unwrap();
    assert!(reference.starts_with("@sha256:"));
    tx.execute("INSERT INTO records VALUES ('s',0,42,?1)", [&reference])
        .unwrap();
    tx.commit().unwrap();
    let source = user_source::read(&path, "s").unwrap().unwrap();
    assert_eq!(source.index, 42);
    assert_eq!(source.text, text);
    assert_eq!(source.text_hash.len(), 64);
}
