use super::super::Session;
use super::info::PersistOutcome;
use super::paths::native_session_path;
use anyhow::Result;

pub(crate) async fn persist_imported_session(
    session: Session,
    data_dir: &std::path::Path,
) -> Result<PersistOutcome> {
    let path = native_session_path(data_dir, &session.id);
    if !crate::session::store::imported::persist(session, &path).await? {
        return Ok(PersistOutcome::Unchanged);
    }
    Ok(PersistOutcome::Saved)
}
