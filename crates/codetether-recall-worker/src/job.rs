//! Commit new projection ranges together with their consumer cursor.

use anyhow::Result;
pub(super) async fn run<H: crate::Host>(id: &str, host: &H) -> Result<()> {
    while let Some(delivery) = host.next(id, "recall", 128).await? {
        let ticket = delivery.ticket;
        let indexed = tokio::task::spawn_blocking(move || H::build(&delivery.session)).await?;
        let Some(indexed) = indexed else {
            return Ok(());
        };
        let workspace = indexed.workspace.clone();
        if host.commit(ticket, serde_json::to_value(&indexed)?).await? {
            let _guard = codetether_recall_store::acquire().await;
            let mut catalog = codetether_recall_store::read_catalog(&workspace, host).await;
            catalog.insert(id);
            codetether_recall_store::write_catalog(&catalog, host).await?;
        }
    }
    Ok(())
}
