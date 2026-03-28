use super::super::Session;
use super::info::PersistOutcome;
use super::paths::native_session_path;
use anyhow::Result;
use tokio::fs;

pub(crate) async fn persist_imported_session(
    session: Session,
    data_dir: &std::path::Path,
) -> Result<PersistOutcome> {
    let path = native_session_path(data_dir, &session.id);
    if let Ok(existing) = fs::read_to_string(&path).await
        && let Ok(existing_session) = serde_json::from_str::<Session>(&existing)
        && existing_session.updated_at >= session.updated_at
    {
        return Ok(PersistOutcome::Unchanged);
    }

    fs::create_dir_all(data_dir).await?;
    fs::write(path, serde_json::to_string_pretty(&session)?).await?;
    Ok(PersistOutcome::Saved)
}
