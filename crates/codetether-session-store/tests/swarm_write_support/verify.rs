//! Database-level evidence after every producer has terminated.
pub fn counts(path: &std::path::Path) -> (i64, i64, i64, String) {
    let db = codetether_session_store::connection::open(path).unwrap();
    let sessions = db
        .query_row(
            "SELECT COUNT(*) FROM sessions WHERE revision=100 AND message_count=100",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let records = db
        .query_row("SELECT COUNT(*) FROM records WHERE kind=0", [], |row| {
            row.get(0)
        })
        .unwrap();
    let commits = db
        .query_row("SELECT COUNT(*) FROM commits", [], |row| row.get(0))
        .unwrap();
    let integrity = db
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .unwrap();
    (sessions, records, commits, integrity)
}
