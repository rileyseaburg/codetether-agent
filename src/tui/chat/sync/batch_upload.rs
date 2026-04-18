//! Single batch upload logic.

use std::path::PathBuf;

use anyhow::Result;
use minio::s3::builders::ObjectContent;
use minio::s3::types::S3Api;

use super::archive_reader::read_chat_archive_batch;
use super::config_types::ChatSyncConfig;
use super::minio_client::ensure_minio_bucket;

pub struct ChatSyncBatch {
    pub bytes: u64,
    pub records: usize,
    pub object_key: String,
    pub next_offset: u64,
}

pub async fn sync_batch(
    client: &minio::s3::Client,
    archive_path: &PathBuf,
    config: &ChatSyncConfig,
    host_tag: &str,
    offset: u64,
) -> Result<Option<ChatSyncBatch>> {
    if !archive_path.exists() { return Ok(None); }
    ensure_minio_bucket(client, &config.bucket).await?;
    let (payload, next_offset, records) = read_chat_archive_batch(archive_path, offset)?;
    if payload.is_empty() { return Ok(None); }
    let ts = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
    let pfx = config.prefix.trim_matches('/');
    let key = if pfx.is_empty() {
        format!("{host_tag}/{ts}-chat-evts-{offset:020}-{next_offset:020}.jsonl")
    } else {
        format!("{pfx}/{host_tag}/{ts}-chat-evts-{offset:020}-{next_offset:020}.jsonl")
    };
    let bytes = payload.len() as u64;
    client.put_object_content(&config.bucket, &key, ObjectContent::from(payload)).send().await?;
    Ok(Some(ChatSyncBatch { bytes, records, object_key: key, next_offset }))
}
