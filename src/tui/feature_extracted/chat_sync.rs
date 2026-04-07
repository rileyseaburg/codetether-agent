//! Chat archive synchronization to S3/MinIO
//!
//! Periodically batches chat messages and uploads to S3-compatible storage.

use anyhow::Result;
use minio::s3::builders::ObjectContent;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;


fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_bool(name: &str) -> Option<bool> {
    let value = env_non_empty(name)?;
    let value = value.to_ascii_lowercase();
    match value.as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_bool_str(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn is_placeholder_secret(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "replace-me" | "changeme" | "change-me" | "your-token" | "your-key"
    )
}

fn env_non_placeholder(name: &str) -> Option<String> {
    env_non_empty(name).filter(|value| !is_placeholder_secret(value))
}

fn vault_extra_string(secrets: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        secrets
            .extra
            .get(*key)
            .and_then(|value| value.as_str())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}

fn vault_extra_bool(secrets: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|key| {
        secrets
            .extra
            .get(*key)
            .and_then(|value| value.as_str())
            .and_then(|s| match s {
                "true" | "1" | "yes" => Some(true),
                "false" | "0" | "no" => Some(false),
                _ => None,
            })
    })
}

fn normalize_minio_endpoint(endpoint: &str) -> String {
    let mut normalized = endpoint.trim().trim_end_matches('/').to_string();
    if let Some(stripped) = normalized.strip_suffix("/login") {
        normalized = stripped.trim_end_matches('/').to_string();
    }
    if !normalized.starts_with("http://") && !normalized.starts_with("https://") {
        normalized = format!("http://{normalized}");
    }
    normalized
}


fn minio_fallback_endpoint(endpoint: &str) -> Option<String> {
    if endpoint.contains(":9001") {
        Some(endpoint.replacen(":9001", ":9000", 1))
    } else {
        None
    }
}


fn sanitize_s3_key_segment(value: &str) -> String {
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
        "unknown".to_string()
    } else {
        cleaned
    }
}


async fn parse_chat_sync_config(
    require_chat_sync: bool,
) -> std::result::Result<Option<ChatSyncConfig>, String> {
    let enabled = env_bool("CODETETHER_CHAT_SYNC_ENABLED").unwrap_or(false);
    if !enabled {
        tracing::info!(
            secure_required = require_chat_sync,
            "Remote chat sync disabled (CODETETHER_CHAT_SYNC_ENABLED is false or unset)"
        );
        if require_chat_sync {
            return Err(
                "CODETETHER_CHAT_SYNC_ENABLED must be true in secure environment".to_string(),
            );
        }
        return Ok(None);
    }

    let vault_secrets = crate::secrets::get_provider_secrets("chat-sync-minio").await;

    let endpoint_raw = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                secrets
                    .base_url
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .or_else(|| vault_extra_string(secrets, &["endpoint", "minio_endpoint"]))
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_ENDPOINT is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.base_url)".to_string()
        })?;
    let endpoint = normalize_minio_endpoint(&endpoint_raw);
    let fallback_endpoint = minio_fallback_endpoint(&endpoint);

    let access_key = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_string(
                    secrets,
                    &[
                        "access_key",
                        "minio_access_key",
                        "username",
                        "key",
                        "api_key",
                    ],
                )
                .or_else(|| {
                    secrets
                        .api_key
                        .as_ref()
                        .map(|value| value.trim().to_string())
                        .filter(|value| !value.is_empty())
                })
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.access_key/api_key)".to_string()
        })?;
    let secret_key = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_string(
                    secrets,
                    &[
                        "secret_key",
                        "minio_secret_key",
                        "password",
                        "secret",
                        "api_secret",
                    ],
                )
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.secret_key)".to_string()
        })?;

    let bucket = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_BUCKET")
        .or_else(|| {
            vault_secrets
                .as_ref()
                .and_then(|secrets| vault_extra_string(secrets, &["bucket", "minio_bucket"]))
        })
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_BUCKET.to_string());
    let prefix = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_PREFIX")
        .or_else(|| {
            vault_secrets
                .as_ref()
                .and_then(|secrets| vault_extra_string(secrets, &["prefix", "minio_prefix"]))
        })
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_PREFIX.to_string())
        .trim_matches('/')
        .to_string();

    let interval_secs = env_non_empty("CODETETHER_CHAT_SYNC_INTERVAL_SECS")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(CHAT_SYNC_DEFAULT_INTERVAL_SECS)
        .clamp(1, CHAT_SYNC_MAX_INTERVAL_SECS);

    let ignore_cert_check = env_bool("CODETETHER_CHAT_SYNC_MINIO_INSECURE_SKIP_TLS_VERIFY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_bool(
                    secrets,
                    &[
                        "insecure_skip_tls_verify",
                        "ignore_cert_check",
                        "skip_tls_verify",
                    ],
                )
            })
        })
        .unwrap_or(false);

    Ok(Some(ChatSyncConfig {
        endpoint,
        fallback_endpoint,
        access_key,
        secret_key,
        bucket,
        prefix,
        interval_secs,
        ignore_cert_check,
    }))
}


fn chat_sync_checkpoint_path(archive_path: &Path, config: &ChatSyncConfig) -> PathBuf {
    let endpoint_tag = sanitize_s3_key_segment(&config.endpoint.replace("://", "-"));
    let bucket_tag = sanitize_s3_key_segment(&config.bucket);
    archive_path.with_file_name(format!(
        "chat_events.minio-sync.{endpoint_tag}.{bucket_tag}.offset"
    ))
}


fn load_chat_sync_offset(checkpoint_path: &Path) -> u64 {
    std::fs::read_to_string(checkpoint_path)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(0)
}


fn store_chat_sync_offset(checkpoint_path: &Path, offset: u64) {
    if let Some(parent) = checkpoint_path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        tracing::warn!(error = %err, path = %parent.display(), "Failed to create chat sync checkpoint directory");
        return;
    }

    if let Err(err) = std::fs::write(checkpoint_path, offset.to_string()) {
        tracing::warn!(error = %err, path = %checkpoint_path.display(), "Failed to persist chat sync checkpoint");
    }
}


fn build_minio_client(endpoint: &str, config: &ChatSyncConfig) -> Result<MinioClient> {
    let base_url: BaseUrl = endpoint.parse()?;
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let client = MinioClientBuilder::new(base_url)
        .provider(Some(Box::new(provider)))
        .ignore_cert_check(Some(config.ignore_cert_check))
        .build()?;
    Ok(client)
}


async fn ensure_minio_bucket(client: &MinioClient, bucket: &str) -> Result<()> {
    let exists = client.bucket_exists(bucket).send().await?;
    if !exists.exists {
        match client.create_bucket(bucket).send().await {
            Ok(_) => {}
            Err(err) => {
                let error_text = err.to_string();
                if !error_text.contains("BucketAlreadyOwnedByYou")
                    && !error_text.contains("BucketAlreadyExists")
                {
                    return Err(anyhow::anyhow!(error_text));
                }
            }
        }
    }
    Ok(())
}


fn read_chat_archive_batch(archive_path: &Path, offset: u64) -> Result<(Vec<u8>, u64, usize)> {
    let metadata = std::fs::metadata(archive_path)?;
    let file_len = metadata.len();
    if offset >= file_len {
        return Ok((Vec::new(), offset, 0));
    }

    let mut file = std::fs::File::open(archive_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let target_bytes = (file_len - offset).min(CHAT_SYNC_MAX_BATCH_BYTES as u64) as usize;
    let mut buffer = vec![0_u8; target_bytes];
    let read = file.read(&mut buffer)?;
    buffer.truncate(read);

    if read == 0 {
        return Ok((Vec::new(), offset, 0));
    }

    // Try to end batches on a newline when there is still more data pending.
    if offset + (read as u64) < file_len {
        if let Some(last_newline) = buffer.iter().rposition(|byte| *byte == b'\n') {
            buffer.truncate(last_newline + 1);
        }

        if buffer.is_empty() {
            let mut rolling = Vec::new();
            let mut temp = [0_u8; 4096];
            loop {
                let n = file.read(&mut temp)?;
                if n == 0 {
                    break;
                }
                rolling.extend_from_slice(&temp[..n]);
                if let Some(pos) = rolling.iter().position(|byte| *byte == b'\n') {
                    rolling.truncate(pos + 1);
                    break;
                }
                if rolling.len() >= CHAT_SYNC_MAX_BATCH_BYTES {
                    break;
                }
            }
            buffer = rolling;
        }
    }

    let next_offset = offset + buffer.len() as u64;
    let records = buffer.iter().filter(|byte| **byte == b'\n').count();
    Ok((buffer, next_offset, records))
}

#[derive(Debug)]

struct ChatSyncBatch {
    bytes: u64,
    records: usize,
    object_key: String,
    next_offset: u64,
}


async fn sync_chat_archive_batch(
    client: &MinioClient,
    archive_path: &Path,
    config: &ChatSyncConfig,
    host_tag: &str,
    offset: u64,
) -> Result<Option<ChatSyncBatch>> {
    if !archive_path.exists() {
        return Ok(None);
    }

    ensure_minio_bucket(client, &config.bucket).await?;

    let (payload, next_offset, records) = read_chat_archive_batch(archive_path, offset)?;
    if payload.is_empty() {
        return Ok(None);
    }

    let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
    let key_prefix = config.prefix.trim_matches('/');
    let object_key = if key_prefix.is_empty() {
        format!("{host_tag}/{timestamp}-chat-events-{offset:020}-{next_offset:020}.jsonl")
    } else {
        format!(
            "{key_prefix}/{host_tag}/{timestamp}-chat-events-{offset:020}-{next_offset:020}.jsonl"
        )
    };

    let bytes = payload.len() as u64;
    let content = ObjectContent::from(payload);
    client
        .put_object_content(&config.bucket, &object_key, content)
        .send()
        .await?;

    Ok(Some(ChatSyncBatch {
        bytes,
        records,
        object_key,
        next_offset,
    }))
}


async fn run_chat_sync_worker(
    tx: mpsc::Sender<ChatSyncUiEvent>,
    archive_path: PathBuf,
    config: ChatSyncConfig,
) {
    let checkpoint_path = chat_sync_checkpoint_path(&archive_path, &config);
    let mut offset = load_chat_sync_offset(&checkpoint_path);
    let host_tag = sanitize_s3_key_segment(
        &std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
    );

    let fallback_label = config
        .fallback_endpoint
        .as_ref()
        .map(|fallback| format!(" (fallback: {fallback})"))
        .unwrap_or_default();

    let _ = tx
        .send(ChatSyncUiEvent::Status(format!(
            "Archive sync enabled → {} / {} every {}s{}",
            config.endpoint, config.bucket, config.interval_secs, fallback_label
        )))
        .await;

    tracing::info!(
        endpoint = %config.endpoint,
        bucket = %config.bucket,
        prefix = %config.prefix,
        interval_secs = config.interval_secs,
        checkpoint = %checkpoint_path.display(),
        archive = %archive_path.display(),
        "Chat sync worker started"
    );

    let mut interval = tokio::time::interval(Duration::from_secs(config.interval_secs));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        let primary_client = match build_minio_client(&config.endpoint, &config) {
            Ok(client) => client,
            Err(err) => {
                let _ = tx
                    .send(ChatSyncUiEvent::Error(format!(
                        "Chat sync client init failed for {}: {err}",
                        config.endpoint
                    )))
                    .await;
                continue;
            }
        };

        let outcome = match sync_chat_archive_batch(
            &primary_client,
            &archive_path,
            &config,
            &host_tag,
            offset,
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(primary_err) => {
                if let Some(fallback_endpoint) = &config.fallback_endpoint {
                    let fallback_client = build_minio_client(fallback_endpoint, &config);
                    match fallback_client {
                        Ok(client) => {
                            let _ = tx
                                .send(ChatSyncUiEvent::Status(format!(
                                    "Primary endpoint failed; retrying with fallback {}",
                                    fallback_endpoint
                                )))
                                .await;
                            sync_chat_archive_batch(
                                &client,
                                &archive_path,
                                &config,
                                &host_tag,
                                offset,
                            )
                            .await
                        }
                        Err(err) => Err(anyhow::anyhow!(
                            "Primary sync error: {primary_err}; fallback init failed: {err}"
                        )),
                    }
                } else {
                    Err(primary_err)
                }
            }
        };

        match outcome {
            Ok(Some(batch)) => {
                offset = batch.next_offset;
                store_chat_sync_offset(&checkpoint_path, offset);
                tracing::info!(
                    bytes = batch.bytes,
                    records = batch.records,
                    object_key = %batch.object_key,
                    next_offset = batch.next_offset,
                    "Chat sync uploaded batch"
                );
                let _ = tx
                    .send(ChatSyncUiEvent::BatchUploaded {
                        bytes: batch.bytes,
                        records: batch.records,
                        object_key: batch.object_key,
                    })
                    .await;
            }
            Ok(None) => {}
            Err(err) => {
                tracing::warn!(error = %err, "Chat sync batch upload failed");
                let _ = tx
                    .send(ChatSyncUiEvent::Error(format!("Chat sync failed: {err}")))
                    .await;
            }
        }
    }
}

// TODO: Incomplete struct definition was truncated during extraction.
// Restore from source branch when integrating this module.
