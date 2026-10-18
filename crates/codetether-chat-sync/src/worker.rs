//! Main chat sync worker loop.

use std::path::PathBuf;
use std::time::Duration;

use super::config_types::ChatSyncConfig;
use super::s3_key::sanitize_s3_key_segment;
use super::s3_key::{chat_sync_checkpoint_path, load_chat_sync_offset};
use super::types::ChatSyncUiEvent;

/// Run the chat sync worker loop.
/// Run periodic archive synchronization.
/// # Arguments
/// Event sender, archive path and configuration.
/// # Returns
/// Remains active, reporting upload outcomes through the sender.
/// # Examples
/// ```text
/// Configure the archive destination, then use run_chat_sync_worker through the chat-sync API.
/// ```
pub async fn run_chat_sync_worker(
    tx: tokio::sync::mpsc::Sender<ChatSyncUiEvent>,
    archive_path: PathBuf,
    config: ChatSyncConfig,
) {
    let cp = chat_sync_checkpoint_path(&archive_path, &config);
    let mut offset = load_chat_sync_offset(&cp);
    let host =
        sanitize_s3_key_segment(&std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into()));
    let _ = tx
        .send(ChatSyncUiEvent::Status(format!(
            "Archive sync → {} / {} every {}s",
            config.endpoint, config.bucket, config.interval_secs,
        )))
        .await;
    let mut iv = tokio::time::interval(Duration::from_secs(config.interval_secs));
    iv.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        iv.tick().await;
        crate::worker_cycle::run(&tx, &archive_path, &config, &host, &mut offset, &cp).await;
    }
}
