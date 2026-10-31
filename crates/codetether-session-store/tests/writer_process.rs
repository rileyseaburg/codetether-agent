//! Separate runtimes coordinate through the same database writer lock.
use codetether_session_store::{connection, coordinator};
use std::process::Command;

#[test]
fn two_processes_commit_to_one_database() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    coordinator::write(&path, |db| {
        db.execute_batch(
            "CREATE TABLE process_probe(worker INTEGER, step INTEGER, PRIMARY KEY(worker, step))",
        )?;
        Ok(())
    })
    .unwrap();
    let mut children: Vec<_> = (0..2)
        .map(|id| {
            Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "writer_process_child"])
                .env("CT_SQLITE_TEST_PATH", &path)
                .env("CT_SQLITE_TEST_ID", id.to_string())
                .spawn()
                .unwrap()
        })
        .collect();
    for child in &mut children {
        assert!(child.wait().unwrap().success());
    }
    let db = connection::open(&path).unwrap();
    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM process_probe", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 50);
}

#[test]
fn writer_process_child() {
    let Some(path) = std::env::var_os("CT_SQLITE_TEST_PATH") else {
        return;
    };
    let path = std::path::PathBuf::from(path);
    let id: i64 = std::env::var("CT_SQLITE_TEST_ID").unwrap().parse().unwrap();
    for step in 0..25 {
        coordinator::write(&path, move |db| {
            db.execute("INSERT INTO process_probe VALUES (?1,?2)", [id, step])?;
            Ok(())
        })
        .unwrap();
    }
}
