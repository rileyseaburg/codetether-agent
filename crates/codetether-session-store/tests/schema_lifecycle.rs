//! Readers must initialize the current database, not trust cached writer state.
use codetether_session_store::{connection, coordinator, metadata};

#[test]
fn reader_upgrades_a_database_after_the_writer_was_initialized() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    let db = connection::open(&path).unwrap();
    db.execute_batch(
        "INSERT INTO sessions VALUES ('retained',1,'{}',0,0);
         DROP TABLE events; DROP TABLE constraints; PRAGMA user_version=1;",
    )
    .unwrap();
    drop(db);
    let db = connection::open(&path).unwrap();
    let version: i64 = db
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(version, 3);
    let count: i64 = db
        .query_row("SELECT count(*) FROM constraints", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    let header: String = db
        .query_row(
            "SELECT body FROM events WHERE session_id='retained'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(header, "{}");
}

#[test]
fn writer_upgrades_before_writing_to_an_already_open_database() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    let db = connection::open(&path).unwrap();
    db.execute_batch("DROP TABLE constraints; PRAGMA user_version=2;")
        .unwrap();
    drop(db);
    coordinator::write(&path, |db| {
        db.execute("DELETE FROM constraints", [])?;
        Ok(())
    })
    .unwrap();
}

#[cfg(unix)]
#[test]
fn recreated_directory_gets_a_new_schema_for_readers_and_catalog() {
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join("sessions");
    let path = directory.join("session.json");
    drop(connection::open(&path).unwrap());
    // Model a restored/recreated data directory without unlinking an open DB.
    std::fs::rename(&directory, root.path().join("previous")).unwrap();
    std::fs::create_dir(&directory).unwrap();
    let db = connection::open(&path).unwrap();
    let count: i64 = db
        .query_row("SELECT count(*) FROM sessions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    coordinator::write(&path, |db| {
        db.execute("INSERT INTO sessions VALUES ('new',1,'{}',0,0)", [])?;
        Ok(())
    })
    .unwrap();
    let rows =
        metadata::listing::read(&directory, None, |header, _| Ok(header.to_owned())).unwrap();
    assert_eq!(rows, ["{}"]);
    let previous =
        rusqlite::Connection::open(root.path().join("previous/sessions.sqlite3")).unwrap();
    let count: i64 = previous
        .query_row("SELECT count(*) FROM sessions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}
