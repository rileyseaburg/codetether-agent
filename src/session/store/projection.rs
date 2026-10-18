//! Native session-path adapter for shared materialized view persistence.
use anyhow::Result;
pub(crate) use codetether_session_store::projection::commit;
use serde_json::Value;
pub(crate) async fn read(id: &str, name: &str) -> Result<Option<Value>> {
    let path = crate::session::Session::session_path(id)?;
    codetether_session_store::projection::read(&path, id, name).await
}
