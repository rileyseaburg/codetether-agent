//! S3 key sanitization and checkpoint path helpers.

use std::path::{Path, PathBuf};

use super::config_types::ChatSyncConfig;

/// Normalize a segment for use in an object key.
/// # Arguments
/// A raw key segment.
/// # Returns
/// ASCII-safe segment or unknown for an empty result.
/// # Examples
/// ```text
/// Configure the archive destination, then use sanitize_s3_key_segment through the chat-sync API.
/// ```
pub fn sanitize_s3_key_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch);
        } else {
            out.push('-');
        }
    }
    let cleaned = out.trim_matches('-').to_string();
    if cleaned.is_empty() {
        "unknown".into()
    } else {
        cleaned
    }
}

/// Derive a checkpoint filename for an archive destination.
/// # Arguments
/// Archive path and destination configuration.
/// # Returns
/// A checkpoint path unique to the normalized endpoint and bucket.
/// # Examples
/// ```text
/// Configure the archive destination, then use chat_sync_checkpoint_path through the chat-sync API.
/// ```
pub fn chat_sync_checkpoint_path(archive_path: &Path, config: &ChatSyncConfig) -> PathBuf {
    let e = sanitize_s3_key_segment(&config.endpoint.replace("://", "-"));
    let b = sanitize_s3_key_segment(&config.bucket);
    archive_path.with_file_name(format!("chat_events.minio-sync.{e}.{b}.offset"))
}

/// Load the last persisted archive offset.
/// # Arguments
/// Checkpoint file path.
/// # Returns
/// Saved offset, or zero for unavailable or invalid content.
/// # Examples
/// ```text
/// Configure the archive destination, then use load_chat_sync_offset through the chat-sync API.
/// ```
pub fn load_chat_sync_offset(checkpoint_path: &Path) -> u64 {
    std::fs::read_to_string(checkpoint_path)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

/// Persist the last completed archive offset.
/// # Arguments
/// Checkpoint path and new byte offset.
/// # Returns
/// Unit; write failures are logged.
/// # Examples
/// ```text
/// Configure the archive destination, then use store_chat_sync_offset through the chat-sync API.
/// ```
pub fn store_chat_sync_offset(checkpoint_path: &Path, offset: u64) {
    if let Err(err) = std::fs::write(checkpoint_path, offset.to_string()) {
        tracing::warn!(error = %err, "Failed to store chat sync offset");
    }
}
