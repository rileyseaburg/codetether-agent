//! S3 key sanitization and checkpoint path helpers.

use std::path::{Path, PathBuf};

use super::config::ChatSyncConfig;

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
    if cleaned.is_empty() { "unknown".into() } else { cleaned }
}

pub fn chat_sync_checkpoint_path(archive_path: &Path, config: &ChatSyncConfig) -> PathBuf {
    let e = sanitize_s3_key_segment(&config.endpoint.replace("://", "-"));
    let b = sanitize_s3_key_segment(&config.bucket);
    archive_path.with_file_name(format!("chat_events.minio-sync.{e}.{b}.offset"))
}

pub fn load_chat_sync_offset(checkpoint_path: &Path) -> u64 {
    std::fs::read_to_string(checkpoint_path)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

pub fn store_chat_sync_offset(checkpoint_path: &Path, offset: u64) {
    if let Err(err) = std::fs::write(checkpoint_path, offset.to_string()) {
        tracing::warn!(error = %err, "Failed to store chat sync offset");
    }
}
