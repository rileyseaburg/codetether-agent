//! Delete authoritative rows before removing the compatibility locator.
use anyhow::Result;
use std::path::Path;
pub(crate) async fn run(path: &Path, id: &str) -> Result<()> {
    let path = path.to_path_buf();
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || {
        if !path.parent().unwrap().join("sessions.sqlite3").exists() {
            return Ok(());
        }
        let db = super::connection::open(&path)?;
        let tx = db.unchecked_transaction()?;
        for table in ["tool_calls", "commits", "views", "projections"] {
            tx.execute(&format!("DELETE FROM {table} WHERE session_id=?1"), [&id])?;
        }
        tx.execute("DELETE FROM sessions WHERE id=?1", [&id])?;
        tx.commit()?;
        anyhow::Ok(())
    })
    .await?
}
