//! MinIO S3 Bus Sink
//!
//! Subscribes to the AgentBus and uploads all messages to MinIO S3
//! as structured JSONL records suitable for LLM pretraining.
//!
//! Each bus message is transformed into a training-friendly record with:
//! - Clear role attribution (system/user/assistant/tool)
//! - Paired tool_call → tool_result sequences
//! - Full, untruncated content (code, file paths, arguments, outputs)
//! - Rich metadata for filtering, deduplication, and curriculum design
//!
//! The output format follows the OpenAI chat-completions schema so it can
//! be fed directly into fine-tuning pipelines (SFT, DPO, RLHF).
//!
//! ## Usage
//!
//! ```rust,ignore
//! use codetether_agent::bus::s3_sink::BusS3Sink;
//!
//! let sink = BusS3Sink::new(
//!     bus.clone(),
//!     "http://localhost:9000",
//!     "access-key",
//!     "secret-key",
//!     "codetether-training",
//!     "bus/",
//! ).await?;
//!
//! // Runs forever, uploading bus messages to S3
//! sink.run().await;
//! ```

use super::{AgentBus, BusEnvelope, BusMessage};
use crate::a2a::types::Part;
use crate::secrets;
use anyhow::{Context, Result};
use chrono::Utc;
use minio::s3::builders::ObjectContent;
use minio::s3::client::{MinioClient, MinioClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task;
use tracing::{debug, error, info, warn};

/// Configuration for the bus S3 sink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusS3SinkConfig {
    /// MinIO/S3 endpoint URL (e.g., "http://localhost:9000")
    pub endpoint: String,
    /// Access key
    pub access_key: String,
    /// Secret key
    pub secret_key: String,
    /// Bucket name for bus logs
    pub bucket: String,
    /// Path prefix within the bucket
    #[serde(default = "default_prefix")]
    pub prefix: String,
    /// Batch size before flushing to S3
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    /// Max batch age in seconds before flushing
    #[serde(default = "default_flush_interval_secs")]
    pub flush_interval_secs: u64,
    /// Whether to use SSL/TLS
    #[serde(default)]
    pub secure: bool,
    /// Whether to ignore certificate errors (for self-signed certs)
    #[serde(default)]
    pub ignore_cert: bool,
}

fn default_prefix() -> String {
    "training/".to_string()
}

fn default_batch_size() -> usize {
    100
}

fn default_flush_interval_secs() -> u64 {
    30
}

impl BusS3SinkConfig {
    /// Create config from environment variables
    ///
    /// Required:
    /// - `MINIO_ENDPOINT` or `CODETETHER_BUS_S3_ENDPOINT`
    /// - `MINIO_ACCESS_KEY` or `CODETETHER_BUS_S3_ACCESS_KEY`
    /// - `MINIO_SECRET_KEY` or `CODETETHER_BUS_S3_SECRET_KEY`
    ///
    /// Optional:
    /// - `CODETETHER_BUS_S3_BUCKET` (default: "codetether-training")
    /// - `CODETETHER_BUS_S3_PREFIX` (default: "bus/")
    pub fn from_env() -> Result<Self> {
        let endpoint = std::env::var("MINIO_ENDPOINT")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_ENDPOINT"))
            .context("MINIO_ENDPOINT or CODETETHER_BUS_S3_ENDPOINT required for bus S3 sink")?;

        let access_key = std::env::var("MINIO_ACCESS_KEY")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_ACCESS_KEY"))
            .context("MINIO_ACCESS_KEY or CODETETHER_BUS_S3_ACCESS_KEY required")?;

        let secret_key = std::env::var("MINIO_SECRET_KEY")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_SECRET_KEY"))
            .context("MINIO_SECRET_KEY or CODETETHER_BUS_S3_SECRET_KEY required")?;

        Ok(Self {
            endpoint,
            access_key,
            secret_key,
            bucket: std::env::var("CODETETHER_BUS_S3_BUCKET")
                .unwrap_or_else(|_| "codetether-training".to_string()),
            prefix: std::env::var("CODETETHER_BUS_S3_PREFIX")
                .unwrap_or_else(|_| "training/".to_string()),
            batch_size: std::env::var("CODETETHER_BUS_S3_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            flush_interval_secs: std::env::var("CODETETHER_BUS_S3_FLUSH_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            secure: std::env::var("MINIO_SECURE")
                .ok()
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
            ignore_cert: std::env::var("MINIO_IGNORE_CERT")
                .ok()
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
        })
    }

    /// Create config by trying multiple credential sources in order:
    ///
    /// 1. Bus-specific env vars (`MINIO_ENDPOINT`, `CODETETHER_BUS_S3_ENDPOINT`)
    /// 2. Chat-sync env vars (`CODETETHER_CHAT_SYNC_MINIO_ENDPOINT`)
    /// 3. Vault secrets at `secret/codetether/providers/chat-sync-minio`
    pub async fn from_env_or_vault() -> Result<Self> {
        // Fast path: original env-only method
        if let Ok(cfg) = Self::from_env() {
            return Ok(cfg);
        }

        // Try chat-sync env vars
        let endpoint = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT");
        let access_key = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY");
        let secret_key = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY");

        if let (Some(ep), Some(ak), Some(sk)) =
            (endpoint.clone(), access_key.clone(), secret_key.clone())
        {
            info!("Bus S3 sink using chat-sync env vars");
            return Ok(Self {
                endpoint: ep,
                access_key: ak,
                secret_key: sk,
                bucket: std::env::var("CODETETHER_BUS_S3_BUCKET")
                    .unwrap_or_else(|_| "codetether-training".to_string()),
                prefix: std::env::var("CODETETHER_BUS_S3_PREFIX")
                    .unwrap_or_else(|_| "training/".to_string()),
                batch_size: 100,
                flush_interval_secs: 30,
                secure: false,
                ignore_cert: false,
            });
        }

        // Try Vault: chat-sync-minio provider
        if let Some(secrets) = secrets::get_provider_secrets("chat-sync-minio").await {
            let ep = secrets
                .base_url
                .clone()
                .or_else(|| vault_extra_str(&secrets, &["endpoint", "minio_endpoint", "url"]))
                .filter(|s| !s.is_empty());
            let ak = vault_extra_str(
                &secrets,
                &["access_key", "access_key_id", "minio_access_key"],
            )
            .or_else(|| secrets.api_key.clone())
            .filter(|s| !s.is_empty());
            let sk = vault_extra_str(
                &secrets,
                &["secret_key", "secret_access_key", "minio_secret_key"],
            )
            .filter(|s| !s.is_empty());

            if let (Some(ep), Some(ak), Some(sk)) = (ep, ak, sk) {
                info!("Bus S3 sink using Vault chat-sync-minio credentials");
                return Ok(Self {
                    endpoint: ep,
                    access_key: ak,
                    secret_key: sk,
                    bucket: std::env::var("CODETETHER_BUS_S3_BUCKET")
                        .unwrap_or_else(|_| "codetether-training".to_string()),
                    prefix: std::env::var("CODETETHER_BUS_S3_PREFIX")
                        .unwrap_or_else(|_| "training/".to_string()),
                    batch_size: 100,
                    flush_interval_secs: 30,
                    secure: false,
                    ignore_cert: false,
                });
            }
        }

        anyhow::bail!(
            "No MinIO credentials found. Set MINIO_ENDPOINT/MINIO_ACCESS_KEY/MINIO_SECRET_KEY, \
             CODETETHER_CHAT_SYNC_MINIO_* env vars, or configure chat-sync-minio in Vault."
        )
    }
}

/// Read an env var, returning `None` if unset or empty.
fn env_non_empty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.is_empty())
}

/// Extract a string value from `ProviderSecrets.extra`, trying multiple key names.
fn vault_extra_str(secrets: &secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(val) = secrets.extra.get(*key)
            && let Some(s) = val.as_str()
            && !s.is_empty()
        {
            return Some(s.to_string());
        }
    }
    None
}

// ─── LLM Pretraining Record ─────────────────────────────────────────────

/// A single training record in OpenAI chat-completions format.
///
/// Each bus envelope maps to one `TrainingRecord` line in the JSONL output.
/// The schema is compatible with OpenAI fine-tuning, Axolotl, and similar
/// pipelines so the data can be used directly for SFT / DPO / RLHF.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainingRecord {
    /// "system" | "user" | "assistant" | "tool"
    role: String,
    /// Primary text content
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Tool calls made by the assistant (only when role == "assistant")
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<TrainingToolCall>>,
    /// Tool call id this result corresponds to (only when role == "tool")
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    /// Tool name (only when role == "tool")
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Envelope metadata for filtering and curriculum design
    metadata: TrainingMetadata,
}

/// Represents a tool call in the assistant's response.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainingToolCall {
    /// Unique id for this tool call
    id: String,
    /// Always "function"
    #[serde(rename = "type")]
    call_type: String,
    /// The function being called
    function: TrainingFunction,
}

/// Function name + arguments inside a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainingFunction {
    name: String,
    arguments: String,
}

/// Provenance metadata attached to every training record.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainingMetadata {
    /// Original BusMessage variant name (snake_case)
    bus_kind: String,
    /// Envelope id
    envelope_id: String,
    /// ISO 8601 timestamp
    timestamp: String,
    /// Hierarchical topic
    topic: String,
    /// Agent that originated the message
    sender_id: String,
    /// Correlation id linking request ↔ response
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    /// Agent loop step number when available
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<usize>,
}

/// Convert a `BusEnvelope` into a `TrainingRecord`.
fn envelope_step(message: &BusMessage) -> Option<usize> {
    match message {
        BusMessage::ToolRequest { step, .. }
        | BusMessage::ToolResponse { step, .. }
        | BusMessage::ToolOutputFull { step, .. }
        | BusMessage::AgentThinking { step, .. }
        | BusMessage::RalphLearning {
            iteration: step, ..
        }
        | BusMessage::RalphProgress {
            iteration: step, ..
        } => Some(*step),
        BusMessage::AgentReady { .. }
        | BusMessage::AgentShutdown { .. }
        | BusMessage::AgentMessage { .. }
        | BusMessage::TaskUpdate { .. }
        | BusMessage::ArtifactUpdate { .. }
        | BusMessage::SharedResult { .. }
        | BusMessage::Heartbeat { .. }
        | BusMessage::RalphHandoff { .. }
        | BusMessage::VoiceSessionStarted { .. }
        | BusMessage::VoiceTranscript { .. }
        | BusMessage::VoiceAgentStateChanged { .. }
        | BusMessage::VoiceSessionEnded { .. } => None,
    }
}

fn envelope_to_training_record(env: &BusEnvelope) -> TrainingRecord {
    let meta = TrainingMetadata {
        bus_kind: bus_message_kind(&env.message),
        envelope_id: env.id.clone(),
        timestamp: env.timestamp.to_rfc3339(),
        topic: env.topic.clone(),
        sender_id: env.sender_id.clone(),
        correlation_id: env.correlation_id.clone(),
        step: envelope_step(&env.message),
    };

    match &env.message {
        // ── Agent lifecycle → system ────────────────────────────────
        BusMessage::AgentReady {
            agent_id,
            capabilities,
        } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Agent `{agent_id}` ready. Capabilities: {}",
                capabilities.join(", ")
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::AgentShutdown { agent_id } => TrainingRecord {
            role: "system".into(),
            content: Some(format!("Agent `{agent_id}` shutting down.")),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        // ── Agent messages → assistant ──────────────────────────────
        BusMessage::AgentMessage { from, to, parts } => {
            let text = parts_to_text(parts);
            TrainingRecord {
                role: "assistant".into(),
                content: Some(format!("[{from} → {to}] {text}")),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                metadata: meta,
            }
        }

        // ── Task/artifact lifecycle → system ────────────────────────
        BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } => {
            let msg = message.as_deref().unwrap_or("");
            TrainingRecord {
                role: "system".into(),
                content: Some(format!("Task `{task_id}` → {state:?}. {msg}")),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                metadata: meta,
            }
        }

        BusMessage::ArtifactUpdate { task_id, artifact } => {
            let artifact_text = parts_to_text(&artifact.parts);
            TrainingRecord {
                role: "system".into(),
                content: Some(format!(
                    "Task `{task_id}` artifact `{}`: {artifact_text}",
                    artifact.artifact_id
                )),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                metadata: meta,
            }
        }

        // ── Shared results → system ────────────────────────────────
        BusMessage::SharedResult { key, value, tags } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Shared result `{key}` [{}]: {}",
                tags.join(", "),
                serde_json::to_string(value).unwrap_or_default()
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        // ── Tool request → assistant with tool_calls ────────────────
        BusMessage::ToolRequest {
            request_id,
            tool_name,
            arguments,
            ..
        } => TrainingRecord {
            role: "assistant".into(),
            content: None,
            tool_calls: Some(vec![TrainingToolCall {
                id: request_id.clone(),
                call_type: "function".into(),
                function: TrainingFunction {
                    name: tool_name.clone(),
                    arguments: serde_json::to_string(arguments).unwrap_or_default(),
                },
            }]),
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        // ── Tool response → tool role ───────────────────────────────
        BusMessage::ToolResponse {
            request_id,
            tool_name,
            result,
            success,
            ..
        } => TrainingRecord {
            role: "tool".into(),
            content: Some(if *success {
                result.clone()
            } else {
                format!("[ERROR] {result}")
            }),
            tool_calls: None,
            tool_call_id: Some(request_id.clone()),
            name: Some(tool_name.clone()),
            metadata: meta,
        },

        // ── Full tool output → tool role (untruncated) ──────────────
        BusMessage::ToolOutputFull {
            agent_id,
            tool_name,
            output,
            success,
            step,
        } => TrainingRecord {
            role: "tool".into(),
            content: Some(if *success {
                format!("[step {step}, agent {agent_id}] {output}")
            } else {
                format!("[step {step}, agent {agent_id}, ERROR] {output}")
            }),
            tool_calls: None,
            tool_call_id: None,
            name: Some(tool_name.clone()),
            metadata: meta,
        },

        // ── Heartbeat → system (filtered out during flush) ──────────
        BusMessage::Heartbeat { .. } => TrainingRecord {
            role: "system".into(),
            content: None,
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        // ── Ralph learnings → system context ────────────────────────
        BusMessage::RalphLearning {
            prd_id,
            story_id,
            iteration,
            learnings,
            context,
        } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Ralph learning (PRD {prd_id}, story {story_id}, iter {iteration}):\n{}\nContext: {}",
                learnings
                    .iter()
                    .map(|l| format!("- {l}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                serde_json::to_string(context).unwrap_or_default()
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::RalphHandoff {
            prd_id,
            from_story,
            to_story,
            context,
            progress_summary,
        } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Ralph handoff (PRD {prd_id}): {from_story} → {to_story}\nSummary: {progress_summary}\nContext: {}",
                serde_json::to_string(context).unwrap_or_default()
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::RalphProgress {
            prd_id,
            passed,
            total,
            iteration,
            status,
        } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Ralph progress (PRD {prd_id}): {passed}/{total} stories passed, iter {iteration}, status: {status}"
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        // ── Agent thinking → assistant reasoning ────────────────────
        BusMessage::AgentThinking {
            agent_id,
            thinking,
            step,
        } => TrainingRecord {
            role: "assistant".into(),
            content: Some(format!("<thinking>\n{thinking}\n</thinking>")),
            tool_calls: None,
            tool_call_id: None,
            name: Some(format!("reasoning.{agent_id}.step_{step}")),
            metadata: meta,
        },

        // ── Voice session lifecycle → system ────────────────────────
        BusMessage::VoiceSessionStarted {
            room_name,
            agent_id,
            voice_id,
        } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Voice session started: room={room_name}, agent={agent_id}, voice={voice_id}"
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::VoiceTranscript {
            room_name,
            text,
            role,
            is_final,
        } => TrainingRecord {
            role: if role == "user" {
                "user".into()
            } else {
                "assistant".into()
            },
            content: Some(format!(
                "[voice:{room_name}{}] {text}",
                if *is_final { " final" } else { "" }
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::VoiceAgentStateChanged { room_name, state } => TrainingRecord {
            role: "system".into(),
            content: Some(format!("Voice agent state: room={room_name} → {state}")),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },

        BusMessage::VoiceSessionEnded { room_name, reason } => TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Voice session ended: room={room_name}, reason={reason}"
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        },
    }
}

type ToolGroupKey = (String, usize, Option<String>);

fn collect_training_records(envelopes: &[BusEnvelope]) -> Vec<TrainingRecord> {
    let mut grouped_records: BTreeMap<ToolGroupKey, Vec<TrainingRecord>> = BTreeMap::new();
    let mut passthrough_records = Vec::new();

    for env in envelopes {
        if matches!(env.message, BusMessage::Heartbeat { .. }) {
            continue;
        }
        let record = envelope_to_training_record(env);
        if let Some(step) = record.metadata.step
            && matches!(
                env.message,
                BusMessage::ToolRequest { .. } | BusMessage::ToolResponse { .. }
            )
        {
            grouped_records
                .entry((
                    record.metadata.sender_id.clone(),
                    step,
                    record.metadata.correlation_id.clone(),
                ))
                .or_default()
                .push(record);
            continue;
        }
        passthrough_records.push(record);
    }

    let mut records = passthrough_records;
    for (_key, mut group) in grouped_records {
        append_training_group(&mut group, &mut records);
    }
    records
}

fn append_training_group(records: &mut [TrainingRecord], output: &mut Vec<TrainingRecord>) {
    if records.is_empty() {
        return;
    }

    let assistant_prefix_len = records
        .iter()
        .take_while(|record| {
            record.role == "assistant"
                && record
                    .tool_calls
                    .as_ref()
                    .is_some_and(|calls| calls.len() == 1)
        })
        .count();
    let tool_suffix_len = records[assistant_prefix_len..]
        .iter()
        .take_while(|record| record.role == "tool" && record.tool_call_id.is_some())
        .count();
    let can_merge = assistant_prefix_len > 0
        && tool_suffix_len > 0
        && assistant_prefix_len + tool_suffix_len == records.len();

    if can_merge {
        let mut merged = records[0].clone();
        let mut tool_calls = Vec::with_capacity(assistant_prefix_len);
        let mut envelope_ids = Vec::with_capacity(assistant_prefix_len);
        let mut contents = Vec::new();

        for record in records.iter().take(assistant_prefix_len) {
            envelope_ids.push(record.metadata.envelope_id.clone());
            if let Some(content) = record
                .content
                .as_ref()
                .filter(|content| !content.is_empty())
            {
                contents.push(content.clone());
            }
            if let Some(mut calls) = record.tool_calls.clone() {
                tool_calls.append(&mut calls);
            }
        }

        merged.tool_calls = Some(tool_calls);
        merged.content = if contents.is_empty() {
            None
        } else {
            Some(contents.join("\n"))
        };
        merged.metadata.bus_kind = "tool_request_batch".into();
        merged.metadata.envelope_id = envelope_ids.join(",");

        output.push(merged);
        output.extend(records.iter().skip(assistant_prefix_len).cloned());
        return;
    }

    output.extend(records.iter().cloned());
}

fn serialize_training_records(records: &[TrainingRecord]) -> Vec<String> {
    records
        .iter()
        .filter_map(|record| serde_json::to_string(record).ok())
        .collect()
}

fn build_s3_key(prefix: &str, now: chrono::DateTime<Utc>) -> String {
    let prefix = if prefix.is_empty() {
        String::new()
    } else if prefix.ends_with('/') {
        prefix.to_string()
    } else {
        format!("{prefix}/")
    };
    let date_path = now.format("%Y/%m/%d/%H").to_string();
    let timestamp = now.format("%Y%m%dT%H%M%S").to_string();
    let uuid = uuid::Uuid::new_v4();
    format!("{prefix}v2/{date_path}/batch_{timestamp}_{uuid}.jsonl")
}

/// Extract the serde tag name from a `BusMessage` variant.
fn bus_message_kind(msg: &BusMessage) -> String {
    serde_json::to_value(msg)
        .ok()
        .and_then(|v| v.get("kind").and_then(|k| k.as_str()).map(String::from))
        .unwrap_or_else(|| "unknown".into())
}

/// Concatenate `Part` items into a single text string.
fn parts_to_text(parts: &[Part]) -> String {
    parts
        .iter()
        .map(|p| match p {
            Part::Text { text } => text.as_str(),
            Part::Data { .. } => "<<data>>",
            Part::File { .. } => "<<file>>",
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ─── S3 Sink ─────────────────────────────────────────────────────────────

/// S3 sink that archives all bus messages as JSONL training records
pub struct BusS3Sink {
    #[allow(dead_code)]
    bus: Arc<AgentBus>,
    client: MinioClient,
    config: BusS3SinkConfig,
    rx: broadcast::Receiver<BusEnvelope>,
}

impl BusS3Sink {
    #[allow(dead_code)]
    /// Create a new bus S3 sink
    pub async fn new(
        bus: Arc<AgentBus>,
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        bucket: &str,
        prefix: &str,
    ) -> Result<Self> {
        let config = BusS3SinkConfig {
            endpoint: endpoint.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            bucket: bucket.to_string(),
            prefix: prefix.to_string(),
            batch_size: 100,
            flush_interval_secs: 30,
            secure: endpoint.starts_with("https"),
            ignore_cert: false,
        };

        Self::from_config(bus, config).await
    }

    /// Create sink from configuration
    pub async fn from_config(bus: Arc<AgentBus>, config: BusS3SinkConfig) -> Result<Self> {
        let endpoint = normalize_endpoint(&config.endpoint, config.secure);

        let base_url: BaseUrl = endpoint.parse().context("Invalid MinIO endpoint URL")?;

        let static_provider = StaticProvider::new(&config.access_key, &config.secret_key, None);

        let client = MinioClientBuilder::new(base_url)
            .provider(Some(static_provider))
            .ignore_cert_check(Some(config.ignore_cert))
            .build()?;

        let rx = bus.tx.subscribe();

        Ok(Self {
            bus,
            client,
            config,
            rx,
        })
    }

    #[allow(dead_code)]
    /// Create sink from environment variables
    pub async fn from_env(bus: Arc<AgentBus>) -> Result<Self> {
        let config = BusS3SinkConfig::from_env()?;
        Self::from_config(bus, config).await
    }

    /// Ensure the bucket exists, creating it if necessary
    pub async fn ensure_bucket(&self) -> Result<()> {
        match self.client.bucket_exists(&self.config.bucket)?.build().send().await {
            Ok(resp) if resp.exists() => {
                debug!(bucket = %self.config.bucket, "S3 bucket exists");
            }
            Ok(_) => {
                info!(bucket = %self.config.bucket, "Creating S3 bucket");
                match self.client.create_bucket(&self.config.bucket)?.build().send().await {
                    Ok(_) => {}
                    Err(e) => {
                        let err_text = e.to_string();
                        if !err_text.contains("BucketAlreadyOwnedByYou")
                            && !err_text.contains("BucketAlreadyExists")
                        {
                            return Err(anyhow::anyhow!("Failed to create bucket: {err_text}"));
                        }
                        debug!(bucket = %self.config.bucket, "Bucket already exists");
                    }
                }
            }
            Err(e) => {
                debug!(error = %e, bucket = %self.config.bucket, "Bucket check returned error (may already exist)");
            }
        }
        Ok(())
    }

    /// Run the sink loop - subscribes to bus and uploads batches to S3
    pub async fn run(mut self) -> Result<()> {
        self.ensure_bucket().await?;

        info!(
            bucket = %self.config.bucket,
            prefix = %self.config.prefix,
            batch_size = self.config.batch_size,
            flush_secs = self.config.flush_interval_secs,
            "Bus S3 sink started (JSONL training record format)"
        );

        let mut batch: Vec<BusEnvelope> = Vec::with_capacity(self.config.batch_size);
        let mut batch_start: Option<String> = None;
        let mut flush_interval = tokio::time::interval(std::time::Duration::from_secs(
            self.config.flush_interval_secs,
        ));

        loop {
            tokio::select! {
                result = self.rx.recv() => {
                    match result {
                        Ok(envelope) => {
                            if batch_start.is_none() {
                                batch_start = Some(envelope.timestamp.to_rfc3339());
                            }
                            batch.push(envelope);

                            if batch.len() >= self.config.batch_size {
                                if let Err(e) = self.flush_batch(&mut batch, &mut batch_start).await {
                                    error!(error = %e, "Failed to flush batch");
                                }
                                // Yield to allow LLM requests priority access to executor
                                task::yield_now().await;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!(skipped = n, "Bus S3 sink lagged, some messages dropped");
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            info!("Bus channel closed, shutting down S3 sink");
                            if !batch.is_empty()
                                && let Err(e) = self.flush_batch(&mut batch, &mut batch_start).await {
                                    error!(error = %e, "Failed to flush final batch");
                                }
                            return Ok(());
                        }
                    }
                }

                _ = flush_interval.tick() => {
                    if !batch.is_empty() {
                        if let Err(e) = self.flush_batch(&mut batch, &mut batch_start).await {
                            error!(error = %e, "Failed to flush batch on interval");
                        }
                        // Yield to allow LLM requests priority access to executor
                        task::yield_now().await;
                    }
                }
            }
        }
    }

    /// Flush the current batch to S3 as JSONL (one training record per line).
    ///
    /// Heartbeats are filtered out as noise. Every other envelope is
    /// transformed into a `TrainingRecord` and serialized as a single JSON
    /// line, making the file directly ingestible by LLM fine-tuning tools.
    async fn flush_batch(
        &self,
        batch: &mut Vec<BusEnvelope>,
        batch_start: &mut Option<String>,
    ) -> Result<()> {
        if batch.is_empty() {
            return Ok(());
        }

        let _start_time = batch_start
            .take()
            .unwrap_or_else(|| Utc::now().to_rfc3339());
        let envelopes = std::mem::take(batch);

        // Build JSONL: buffer tool requests/responses by agent step and
        // correlation id so separate turns do not merge when a sender reuses
        // the same step number. Legacy producers with `None` correlation ids
        // still group together compatibly.
        let records = collect_training_records(&envelopes);
        let lines = serialize_training_records(&records);
        if lines.is_empty() {
            return Ok(());
        }

        let count = lines.len();
        let jsonl = lines.join("\n");

        // S3 key: prefix/v2/YYYY/MM/DD/HH/batch_YYYYMMDDTHHMMSS_uuid.jsonl
        let now = Utc::now();
        let s3_key = build_s3_key(&self.config.prefix, now);

        let content = ObjectContent::from(jsonl.into_bytes());

        match self.client.put_object_content(&self.config.bucket, &s3_key, content)?.build().send().await {
            Ok(_) => {
                info!(
                    bucket = %self.config.bucket,
                    key = %s3_key,
                    records = count,
                    "Uploaded training records to S3"
                );
            }
            Err(e) => {
                error!(
                    bucket = %self.config.bucket,
                    key = %s3_key,
                    error = %e,
                    "Failed to upload training records to S3"
                );
                return Err(anyhow::anyhow!("S3 upload failed: {e}"));
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.config.bucket
    }

    #[allow(dead_code)]
    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.config.prefix
    }
}

/// Normalize endpoint URL (ensure protocol, remove trailing slash)
fn normalize_endpoint(endpoint: &str, secure: bool) -> String {
    let endpoint = endpoint.trim_end_matches('/');

    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.to_string()
    } else if secure {
        format!("https://{endpoint}")
    } else {
        format!("http://{endpoint}")
    }
}

/// Spawn the bus S3 sink in a background task.
///
/// The S3 sink runs non-blocking in its own task, processing batches
/// of training records. It yields periodically to ensure LLM requests
/// have priority for CPU/network resources.
///
/// Errors are logged but do not crash the application.
pub fn spawn_bus_s3_sink(bus: Arc<AgentBus>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        match BusS3SinkConfig::from_env_or_vault().await {
            Ok(config) => match BusS3Sink::from_config(bus, config).await {
                Ok(sink) => {
                    if let Err(e) = sink.run().await {
                        error!(error = %e, "Bus S3 sink failed");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Bus S3 sink failed to initialize");
                }
            },
            Err(e) => {
                warn!(
                    error = %e,
                    "Bus S3 sink not configured - set MINIO_*/CODETETHER_CHAT_SYNC_MINIO_* env vars or configure chat-sync-minio in Vault"
                );
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_normalize_endpoint() {
        assert_eq!(
            normalize_endpoint("localhost:9000", false),
            "http://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("localhost:9000", true),
            "https://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("http://localhost:9000/", false),
            "http://localhost:9000"
        );
        assert_eq!(
            normalize_endpoint("https://minio.example.com/", true),
            "https://minio.example.com"
        );
    }

    #[test]
    fn test_config_defaults() {
        let config = BusS3SinkConfig {
            endpoint: "http://localhost:9000".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            bucket: "test".to_string(),
            prefix: default_prefix(),
            batch_size: default_batch_size(),
            flush_interval_secs: default_flush_interval_secs(),
            secure: false,
            ignore_cert: false,
        };

        assert_eq!(config.prefix, "training/");
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.flush_interval_secs, 30);
    }

    #[test]
    fn test_training_record_tool_request() {
        let env = BusEnvelope {
            id: "env-1".into(),
            topic: "tools.read_file".into(),
            sender_id: "agent-0".into(),
            correlation_id: Some("corr-1".into()),
            timestamp: Utc::now(),
            message: BusMessage::ToolRequest {
                request_id: "req-1".into(),
                agent_id: "agent-0".into(),
                tool_name: "read_file".into(),
                arguments: serde_json::json!({"path": "/src/main.rs"}),
                step: 1,
            },
        };

        let record = envelope_to_training_record(&env);
        assert_eq!(record.role, "assistant");
        assert!(record.content.is_none());
        let calls = record.tool_calls.unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].function.name, "read_file");
        assert_eq!(calls[0].call_type, "function");
        assert_eq!(record.metadata.bus_kind, "tool_request");
    }

    #[test]
    fn test_training_record_tool_response() {
        let env = BusEnvelope {
            id: "env-2".into(),
            topic: "tools.read_file".into(),
            sender_id: "agent-0".into(),
            correlation_id: Some("corr-1".into()),
            timestamp: Utc::now(),
            message: BusMessage::ToolResponse {
                request_id: "req-1".into(),
                agent_id: "agent-0".into(),
                tool_name: "read_file".into(),
                result: "fn main() {}".into(),
                success: true,
                step: 1,
            },
        };

        let record = envelope_to_training_record(&env);
        assert_eq!(record.role, "tool");
        assert_eq!(record.content.as_deref(), Some("fn main() {}"));
        assert_eq!(record.tool_call_id.as_deref(), Some("req-1"));
        assert_eq!(record.name.as_deref(), Some("read_file"));
        assert_eq!(record.metadata.bus_kind, "tool_response");
    }

    #[test]
    fn test_training_record_agent_message() {
        let env = BusEnvelope {
            id: "env-3".into(),
            topic: "agent.planner".into(),
            sender_id: "coder".into(),
            correlation_id: None,
            timestamp: Utc::now(),
            message: BusMessage::AgentMessage {
                from: "coder".into(),
                to: "planner".into(),
                parts: vec![Part::Text {
                    text: "I fixed the bug".into(),
                }],
            },
        };

        let record = envelope_to_training_record(&env);
        assert_eq!(record.role, "assistant");
        assert!(
            record
                .content
                .as_deref()
                .unwrap()
                .contains("I fixed the bug")
        );
        assert!(
            record
                .content
                .as_deref()
                .unwrap()
                .contains("[coder → planner]")
        );
    }

    #[test]
    fn test_heartbeat_skipped_role() {
        let env = BusEnvelope {
            id: "env-4".into(),
            topic: "broadcast".into(),
            sender_id: "agent-0".into(),
            correlation_id: None,
            timestamp: Utc::now(),
            message: BusMessage::Heartbeat {
                agent_id: "agent-0".into(),
                status: "ok".into(),
            },
        };

        let record = envelope_to_training_record(&env);
        // Heartbeats produce a record but flush_batch filters them out
        assert_eq!(record.role, "system");
        assert!(record.content.is_none());
    }

    #[test]
    fn test_tool_grouping_keeps_different_correlation_ids_separate() {
        let envelopes = vec![
            tool_request_envelope("env-1", "req-1", Some("turn-1")),
            tool_response_envelope("env-2", "req-1", Some("turn-1")),
            tool_request_envelope("env-3", "req-2", Some("turn-2")),
            tool_response_envelope("env-4", "req-2", Some("turn-2")),
        ];

        let records = collect_training_records(&envelopes);
        let assistant_batches: Vec<_> = records
            .iter()
            .filter(|record| record.metadata.bus_kind == "tool_request_batch")
            .collect();

        assert_eq!(records.len(), 4);
        assert_eq!(assistant_batches.len(), 2);
        assert_eq!(
            assistant_batches[0].metadata.correlation_id.as_deref(),
            Some("turn-1")
        );
        assert_eq!(
            assistant_batches[1].metadata.correlation_id.as_deref(),
            Some("turn-2")
        );
        assert_eq!(assistant_batches[0].tool_calls.as_ref().unwrap().len(), 1);
        assert_eq!(assistant_batches[1].tool_calls.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_tool_grouping_keeps_legacy_none_correlation_compatible() {
        let envelopes = vec![
            tool_request_envelope("env-1", "req-1", None),
            tool_request_envelope("env-2", "req-2", None),
            tool_response_envelope("env-3", "req-1", None),
            tool_response_envelope("env-4", "req-2", None),
        ];

        let records = collect_training_records(&envelopes);
        let assistant_batches: Vec<_> = records
            .iter()
            .filter(|record| record.metadata.bus_kind == "tool_request_batch")
            .collect();

        assert_eq!(records.len(), 3);
        assert_eq!(assistant_batches.len(), 1);
        assert!(assistant_batches[0].metadata.correlation_id.is_none());
        assert_eq!(assistant_batches[0].tool_calls.as_ref().unwrap().len(), 2);
    }

    fn tool_request_envelope(
        envelope_id: &str,
        request_id: &str,
        correlation_id: Option<&str>,
    ) -> BusEnvelope {
        BusEnvelope {
            id: envelope_id.into(),
            topic: "tools.read_file".into(),
            sender_id: "agent-0".into(),
            correlation_id: correlation_id.map(str::to_string),
            timestamp: Utc::now(),
            message: BusMessage::ToolRequest {
                request_id: request_id.into(),
                agent_id: "agent-0".into(),
                tool_name: "read_file".into(),
                arguments: Value::Null,
                step: 1,
            },
        }
    }

    fn tool_response_envelope(
        envelope_id: &str,
        request_id: &str,
        correlation_id: Option<&str>,
    ) -> BusEnvelope {
        BusEnvelope {
            id: envelope_id.into(),
            topic: "tools.read_file".into(),
            sender_id: "agent-0".into(),
            correlation_id: correlation_id.map(str::to_string),
            timestamp: Utc::now(),
            message: BusMessage::ToolResponse {
                request_id: request_id.into(),
                agent_id: "agent-0".into(),
                tool_name: "read_file".into(),
                result: "ok".into(),
                success: true,
                step: 1,
            },
        }
    }
}
