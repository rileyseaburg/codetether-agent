//! Bounded blocking load for optional legacy recall migration.

const MAX_SOURCE_BYTES: u64 = 8 * 1024 * 1024;

pub(super) async fn bounded<H: crate::Host>(session_id: &str, host: &H) -> Option<H::Session> {
    let path = host.session_path(session_id).ok()?;
    let metadata = tokio::fs::metadata(&path).await.ok()?;
    if metadata.len() > MAX_SOURCE_BYTES {
        tracing::debug!(target: "codetether_agent::session::recall::backfill_load",
            session_id,
            bytes = metadata.len(),
            "skipping oversized recall backfill"
        );
        return None;
    }
    Some(host.load_tail(session_id, 512).await.ok()?)
}

#[cfg(test)]
mod tests {
    use super::MAX_SOURCE_BYTES;

    #[test]
    fn background_source_cap_is_small() {
        assert_eq!(MAX_SOURCE_BYTES, 8 * 1024 * 1024);
    }
}
