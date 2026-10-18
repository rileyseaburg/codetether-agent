//! Native validation before prepared-context sidecar storage.
use crate::session::Session;
use anyhow::Result;
pub(super) use codetether_prepared_context::Prepared;
pub(super) async fn read_id(session_id: &str) -> Option<Prepared> {
    let path = path(session_id).ok()?;
    codetether_prepared_context::read(&path).await
}
pub(super) async fn remove(session_id: &str) {
    if let Ok(path) = path(session_id) {
        codetether_prepared_context::remove(&path).await;
    }
}
pub(super) async fn write(session_id: &str, value: &Prepared) -> Result<()> {
    let path = path(session_id)?;
    codetether_prepared_context::write(&path, value).await
}

fn path(session_id: &str) -> Result<std::path::PathBuf> {
    Session::session_path(session_id)?;
    Ok(Session::sessions_dir()?
        .join("rlm")
        .join(format!("{session_id}.json")))
}
