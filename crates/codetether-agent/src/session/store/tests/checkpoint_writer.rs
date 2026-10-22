//! External writer used to verify a checkpoint's SQLite snapshot isolation.
pub(super) fn replace(path: &std::path::Path, id: &str) -> anyhow::Result<()> {
    let writer = rusqlite::Connection::open(path)?;
    let body = serde_json::to_string(&(
        super::super::message("after"),
        codetether_context_policy::pages::PageKind::Conversation,
    ))?;
    writer.execute(
        "UPDATE records SET body=?1 WHERE session_id=?2 AND kind=0 AND seq=100",
        rusqlite::params![body, id],
    )?;
    Ok(())
}
pub(super) fn remove(path: &std::path::Path, id: &str) -> anyhow::Result<()> {
    rusqlite::Connection::open(path)?.execute(
        "DELETE FROM records WHERE session_id=?1 AND kind=0 AND seq=64",
        [id],
    )?;
    Ok(())
}
