//! Main chat sync worker loop.

use std::path::PathBuf;
use std::time::Duration;

use super::batch_upload::sync_batch;
use super::config_types::ChatSyncConfig;
use super::minio_client::build_minio_client;
use super::s3_key::{chat_sync_checkpoint_path, load_chat_sync_offset, store_chat_sync_offset};
use super::s3_key::sanitize_s3_key_segment;
use super::types::ChatSyncUiEvent;

/// Run the chat sync worker loop.
pub async fn run_chat_sync_worker(
    tx: tokio::sync::mpsc::Sender<ChatSyncUiEvent>,
    archive_path: PathBuf,
    config: ChatSyncConfig,
) {
    let cp = chat_sync_checkpoint_path(&archive_path, &config);
    let mut offset = load_chat_sync_offset(&cp);
    let host = sanitize_s3_key_segment(
        &std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into()),
    );
    let _ = tx.send(ChatSyncUiEvent::Status(format!(
        "Archive sync → {} / {} every {}s", config.endpoint, config.bucket, config.interval_secs,
    ))).await;
    let mut iv = tokio::time::interval(Duration::from_secs(config.interval_secs));
    iv.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        iv.tick().await;
        let client = match build_minio_client(&config.endpoint, &config) {
            Ok(c) => c,
            Err(e) => { let _ = tx.send(ChatSyncUiEvent::Error(format!("client init: {e}"))).await; continue; }
        };
        match sync_batch(&client, &archive_path, &config, &host, offset).await {
            Ok(Some(b)) => {
                offset = b.next_offset;
                store_chat_sync_offset(&cp, offset);
                let _ = tx.send(ChatSyncUiEvent::BatchUploaded {
                    bytes: b.bytes, records: b.records, object_key: b.object_key,
                }).await;
            }
            Ok(None) => {}
            Err(e) => { let _ = tx.send(ChatSyncUiEvent::Error(format!("sync: {e}"))).await; }
        }
    }
}
