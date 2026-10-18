//! One upload attempt, offset update and UI event reporting.
use crate::{
    batch_upload::sync_batch, config_types::ChatSyncConfig, minio_client::build_minio_client,
    s3_key::store_chat_sync_offset, types::ChatSyncUiEvent,
};
use std::path::PathBuf;
pub(super) async fn run(
    tx: &tokio::sync::mpsc::Sender<ChatSyncUiEvent>,
    archive_path: &PathBuf,
    config: &ChatSyncConfig,
    host: &str,
    offset: &mut u64,
    cp: &PathBuf,
) {
    let client = match build_minio_client(&config.endpoint, &config) {
        Ok(c) => c,
        Err(e) => {
            let _ = tx
                .send(ChatSyncUiEvent::Error(format!("client init: {e}")))
                .await;
            return;
        }
    };
    match sync_batch(&client, archive_path, config, host, *offset).await {
        Ok(Some(b)) => {
            *offset = b.next_offset;
            store_chat_sync_offset(cp, *offset);
            let _ = tx
                .send(ChatSyncUiEvent::BatchUploaded {
                    bytes: b.bytes,
                    records: b.records,
                    object_key: b.object_key,
                })
                .await;
        }
        Ok(None) => {}
        Err(e) => {
            let _ = tx.send(ChatSyncUiEvent::Error(format!("sync: {e}"))).await;
        }
    }
}
