//! Bounded blocking load for optional legacy recall migration.

use crate::session::Session;

const MAX_SOURCE_BYTES: u64 = 8 * 1024 * 1024;

pub(super) async fn bounded(session_id: &str) -> Option<Session> {
    let path = Session::session_path(session_id).ok()?;
    let metadata = tokio::fs::metadata(&path).await.ok()?;
    if metadata.len() > MAX_SOURCE_BYTES {
        tracing::debug!(
            session_id,
            bytes = metadata.len(),
            "skipping oversized recall backfill"
        );
        return None;
    }
    let bytes = tokio::fs::read(path).await.ok()?;
    tokio::task::spawn_blocking(move || decode(&bytes))
        .await
        .ok()?
}

fn decode(bytes: &[u8]) -> Option<Session> {
    let mut session: Session = serde_json::from_slice(bytes).ok()?;
    session.normalize_sidecars();
    Some(session)
}

#[cfg(test)]
mod tests {
    use super::MAX_SOURCE_BYTES;

    #[test]
    fn background_source_cap_is_small() {
        assert_eq!(MAX_SOURCE_BYTES, 8 * 1024 * 1024);
    }
}
