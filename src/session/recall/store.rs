//! Transactional orchestration for recall sidecars and catalogs.

use super::indexed_session::IndexedSession;
use anyhow::Result;

pub(super) async fn upsert(indexed: IndexedSession) -> Result<()> {
    let _guard = super::store_lock::acquire().await;
    if super::tombstone::contains(&indexed.session_id) {
        return Ok(());
    }
    if !crate::session::Session::session_path(&indexed.session_id)?.exists() {
        return Ok(());
    }
    if super::store_freshness::stale(&indexed).await {
        return Ok(());
    }
    super::session_io::write(&indexed).await?;
    let mut catalog = super::catalog_io::read(&indexed.workspace).await;
    catalog.insert(&indexed.session_id);
    super::catalog_io::write(&catalog).await?;
    let key = super::paths::catalog(&indexed.workspace)?;
    super::cache::update(&key, indexed);
    Ok(())
}

pub(super) async fn remove(session_id: &str) -> Result<()> {
    let _guard = super::store_lock::acquire().await;
    let Some(indexed) = super::session_io::read(session_id).await else {
        super::session_io::remove(session_id).await?;
        super::cache::remove_all(session_id);
        return Ok(());
    };
    super::session_io::remove(session_id).await?;
    let mut catalog = super::catalog_io::read(&indexed.workspace).await;
    catalog.remove(session_id);
    super::catalog_io::write(&catalog).await?;
    let key = super::paths::catalog(&indexed.workspace)?;
    super::cache::remove(&key, session_id);
    Ok(())
}
