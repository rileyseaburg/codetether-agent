//! Database setup and deterministic writer holds for coordination tests.
use codetether_session_store::{connection, coordinator};
use std::path::{Path, PathBuf};

pub fn setup() -> (tempfile::TempDir, PathBuf) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("fixture.json");
    coordinator::write(&path, |db| {
        db.execute_batch("CREATE TABLE writer_probe(seq INTEGER PRIMARY KEY, value INTEGER)")?;
        Ok(())
    })
    .unwrap();
    (root, path)
}

pub fn values(path: &Path) -> Vec<i64> {
    let db = connection::open(path).unwrap();
    let mut query = db
        .prepare("SELECT value FROM writer_probe ORDER BY seq")
        .unwrap();
    query
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}
