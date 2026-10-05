//! Commit new projection ranges together with their consumer cursor.
use crate::session::store::{cursor, projection};
use anyhow::Result;
pub(super) async fn run(id: &str) -> Result<()> {
    while let Some(delivery) = cursor::next(id, "recall", 128).await? {
        let ticket = delivery.ticket;
        let indexed =
            tokio::task::spawn_blocking(move || super::super::build::session(&delivery.session))
                .await?;
        let Some(indexed) = indexed else {
            return Ok(());
        };
        let workspace = indexed.workspace.clone();
        if projection::commit(ticket, serde_json::to_value(&indexed)?).await? {
            let _guard = super::super::store_lock::acquire().await;
            let mut catalog = super::super::catalog_io::read(&workspace).await;
            catalog.insert(id);
            super::super::catalog_io::write(&catalog).await?;
        }
    }
    Ok(())
}
