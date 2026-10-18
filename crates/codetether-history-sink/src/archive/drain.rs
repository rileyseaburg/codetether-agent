//! Upload immutable ranges before committing their durable acknowledgement.
use super::ArchiveSource;
use crate::HistorySinkConfig;

pub(super) async fn drain<H: ArchiveSource>(
    id: &str,
    config: &HistorySinkConfig,
) -> anyhow::Result<()> {
    while let Some(delivery) = H::next(id, "archive", 128).await? {
        let body = crate::encode_jsonl_delta(H::messages(&delivery.session), 0)?;
        let ticket = delivery.ticket;
        let key = format!(
            "{id}/chunks/{:020}-{:020}-{}",
            ticket.from, ticket.to, ticket.generation
        );
        crate::upload_encoded_history(config, &key, body.into_bytes()).await?;
        let view = serde_json::json!({ "documents": [{
            "start": ticket.from, "end": ticket.to,
            "bucket": config.bucket, "key": config.object_key(&key)
        }] });
        codetether_session_store::projection::commit(ticket, view).await?;
    }
    Ok(())
}
