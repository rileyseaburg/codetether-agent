//! Immutable archive chunks; failed uploads never advance the durable cursor.
use crate::session::{Session, history_sink::HistorySinkConfig};
mod queue;
pub(crate) fn schedule(session: &Session) {
    let Some(config) = session.metadata.history_sink.clone() else {
        return;
    };
    let id = session.id.clone();
    if !queue::start(&id) {
        return;
    }
    tokio::spawn(async move {
        loop {
            if let Err(error) = drain(&id, &config).await {
                tracing::warn!(session_id = %id, %error, "incremental history upload failed");
                queue::remove(&id);
                break;
            }
            if !queue::again(&id) {
                break;
            }
        }
    });
}
async fn drain(id: &str, config: &HistorySinkConfig) -> anyhow::Result<()> {
    while let Some(delivery) = super::cursor::next(id, "archive", 128).await? {
        let body = crate::session::history_sink::encode_jsonl_delta(&delivery.session.messages, 0)?;
        let ticket = delivery.ticket;
        let key = format!(
            "{id}/chunks/{:020}-{:020}-{}",
            ticket.from, ticket.to, ticket.generation
        );
        crate::session::history_sink::upload_encoded_history(config, &key, body.into_bytes())
            .await?;
        let view = serde_json::json!({ "documents": [{
            "start": ticket.from, "end": ticket.to,
            "bucket": config.bucket, "key": config.object_key(&key)
        }] });
        super::projection::commit(ticket, view).await?;
    }
    Ok(())
}
