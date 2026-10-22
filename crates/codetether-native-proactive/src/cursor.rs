//! Rebuild recent working memory first when a summary policy has a new cursor.
use crate::Session;
use codetether_session_store::{Delivery, consumer, legacy, lifecycle};
pub(super) async fn next<S: Session>(
    id: &str,
    name: &str,
    limit: usize,
) -> anyhow::Result<Option<Delivery<S>>> {
    let path = lifecycle::session_path(id)?;
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || {
        legacy::ensure::<S>(&path)?;
        // Eight recent summary blocks restore current work without replaying
        // an entire multi-thousand-turn archive through the model on upgrade.
        // Older source remains available through normal history retrieval.
        consumer::start_at_tail(&path, &id, &name, 128, 16)?;
        consumer::next::<S>(&path, &id, &name, limit)
    })
    .await?
}
