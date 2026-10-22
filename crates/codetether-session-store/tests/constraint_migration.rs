use codetether_context_policy::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_store::{connection, constraints, schema};
use rusqlite::Connection;

#[test]
fn version_two_excerpts_are_rebuilt_from_full_canonical_sources() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.json");
    let mut db = Connection::open(dir.path().join("sessions.sqlite3")).unwrap();
    db.execute_batch(schema::SQL).unwrap();
    db.execute("INSERT INTO sessions VALUES ('s',1,'{}',1,0)", [])
        .unwrap();
    let text = format!(
        "first line\n{}\nNEVER DROP THE FINAL CORRECTION",
        "🦀".repeat(20000)
    );
    let message = Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.clone() }],
    };
    let original = serde_json::to_string(&(message, PageKind::Constraint)).unwrap();
    let tx = db.transaction().unwrap();
    let blob = codetether_session_store::blobs::put(&tx, &original).unwrap();
    tx.execute("INSERT INTO records VALUES ('s',0,0,?1)", [&blob])
        .unwrap();
    tx.execute("INSERT INTO constraints VALUES ('s',0,'first line')", [])
        .unwrap();
    tx.pragma_update(None, "user_version", 2).unwrap();
    tx.commit().unwrap();
    drop(db);
    let db = connection::open(&path).unwrap();
    let records = constraints::read(&db, "s", 1).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].text, text);
    let stored: String = db
        .query_row("SELECT body FROM records WHERE session_id='s'", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(stored, blob);
    assert_eq!(
        codetether_session_store::blobs::get(&db, stored).unwrap(),
        original
    );
    assert_eq!(
        db.pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap(),
        3
    );
    db.execute(
        "UPDATE constraints SET excerpt=json_set(excerpt,'$.text','altered source')",
        [],
    )
    .unwrap();
    assert!(
        constraints::read(&db, "s", 1)
            .unwrap_err()
            .to_string()
            .contains("integrity mismatch")
    );
}
