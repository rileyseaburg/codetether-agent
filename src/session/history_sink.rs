//! Durable MinIO/S3 sink for pure chat history.
//!
//! ## Why a second sink?
//!
//! Phase A of the history/context refactor makes [`Session::messages`] a
//! pure, append-only record of *what happened*. The local JSON file at
//! `~/.local/share/codetether/sessions/{id}.json` is the authoritative
//! resume cache, but it also gets rewritten on every `save()` — so a
//! crash between saves or a laptop loss evicts the record entirely.
//!
//! The history sink streams the pure transcript to a MinIO-compatible
//! S3 bucket as an **append-only JSONL** object, one line per message.
//! This serves three purposes that the Liu et al.
//! (arXiv:2512.22087), Meta-Harness (arXiv:2603.28052), and ClawVM
//! (arXiv:2604.10352) references all emphasise:
//!
//! 1. **Durable archive** for `session_recall` across machines and
//!    crashes.
//! 2. **Filesystem-as-history substrate** — the virtual
//!    `context_browse` files served in Phase B are backed by these
//!    objects.
//! 3. **Pointer-residency backing store** — when a
//!    [`ResidencyLevel::Pointer`](super::pages) page is selected, its
//!    handle points here.
//!
//! The sink is **env-gated** and **non-fatal**. When the endpoint isn't
//! configured or the upload fails, the session continues; a single
//! `tracing::warn!` records the error and the local file remains
//! authoritative for resume.
//!
//! ## Configuration
//!
//! Reads five environment variables at session load / first save:
//!
//! | Variable                                 | Required? | Purpose                          |
//! |------------------------------------------|-----------|----------------------------------|
//! | `CODETETHER_HISTORY_S3_ENDPOINT`         | Yes       | MinIO/S3 endpoint URL            |
//! | `CODETETHER_HISTORY_S3_BUCKET`           | Yes       | Bucket name                      |
//! | `CODETETHER_HISTORY_S3_PREFIX`           | No        | Key prefix (default `history/`)  |
//! | `CODETETHER_HISTORY_S3_ACCESS_KEY`       | Yes       | S3 access key                    |
//! | `CODETETHER_HISTORY_S3_SECRET_KEY`       | Yes       | S3 secret key                    |
//!
//! When any *required* variable is unset, [`HistorySinkConfig::from_env`]
//! returns `Ok(None)` and the sink is silently disabled.
//!
//! ## Examples
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::history_sink::HistorySinkConfig;
//!
//! match HistorySinkConfig::from_env() {
//!     Ok(Some(config)) => {
//!         assert!(!config.endpoint.is_empty());
//!     }
//!     Ok(None) => {
//!         // Env vars not set; sink disabled.
//!     }
//!     Err(e) => eprintln!("history sink misconfigured: {e}"),
//! }
//! # });
//! ```

use anyhow::{Context, Result};
use minio::s3::builders::ObjectContent;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::provider::Message;
use crate::session::faults::Fault;

/// Object-key suffix where the full transcript lives, relative to
/// [`HistorySinkConfig::prefix`].
const HISTORY_OBJECT_NAME: &str = "history.jsonl";

/// Default object-key prefix when
/// [`HistorySinkConfig::CODETETHER_HISTORY_S3_PREFIX`] is unset.
///
/// [`HistorySinkConfig::CODETETHER_HISTORY_S3_PREFIX`]: HistorySinkConfig
const DEFAULT_PREFIX: &str = "history/";

/// Durable history-sink configuration.
///
/// Intentionally owned (not borrowed) so the config can be handed to a
/// `tokio::spawn` fire-and-forget upload without tying its lifetime to
/// the `Session`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::history_sink::HistorySinkConfig;
///
/// let cfg = HistorySinkConfig {
///     endpoint: "http://localhost:9000".to_string(),
///     access_key: "minioadmin".to_string(),
///     secret_key: "minioadmin".to_string(),
///     bucket: "codetether".to_string(),
///     prefix: "history/".to_string(),
/// };
/// assert_eq!(cfg.bucket, "codetether");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySinkConfig {
    /// MinIO / S3 endpoint URL, e.g. `http://localhost:9000`.
    pub endpoint: String,
    /// S3 access key.
    pub access_key: String,
    /// S3 secret key.
    pub secret_key: String,
    /// Destination bucket.
    pub bucket: String,
    /// Key prefix under the bucket. Always ends with `/`.
    #[serde(default = "default_prefix")]
    pub prefix: String,
}

fn default_prefix() -> String {
    DEFAULT_PREFIX.to_string()
}

impl HistorySinkConfig {
    /// Load the config from environment variables.
    ///
    /// Returns `Ok(None)` when any required variable is unset — callers
    /// treat that as *sink disabled* and skip the upload. Returns
    /// `Err(_)` only when an env variable is present but malformed.
    ///
    /// # Errors
    ///
    /// Never in the current implementation; reserved for future URL
    /// / region validation.
    pub fn from_env() -> Result<Option<Self>> {
        let endpoint = match std::env::var("CODETETHER_HISTORY_S3_ENDPOINT") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let bucket = match std::env::var("CODETETHER_HISTORY_S3_BUCKET") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let access_key = match std::env::var("CODETETHER_HISTORY_S3_ACCESS_KEY") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let secret_key = match std::env::var("CODETETHER_HISTORY_S3_SECRET_KEY") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let prefix = std::env::var("CODETETHER_HISTORY_S3_PREFIX")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| if s.ends_with('/') { s } else { format!("{s}/") })
            .unwrap_or_else(default_prefix);

        Ok(Some(Self {
            endpoint,
            access_key,
            secret_key,
            bucket,
            prefix,
        }))
    }

    /// Object key for the full transcript of `session_id`.
    ///
    /// Example: `history/1234-abcd/history.jsonl`.
    pub fn object_key(&self, session_id: &str) -> String {
        format!("{}{session_id}/{HISTORY_OBJECT_NAME}", self.prefix)
    }
}

/// Build a MinIO client from a [`HistorySinkConfig`].
fn build_client(config: &HistorySinkConfig) -> Result<MinioClient> {
    let base_url = BaseUrl::from_str(&config.endpoint)
        .with_context(|| format!("Invalid MinIO endpoint: {}", config.endpoint))?;
    let creds = StaticProvider::new(&config.access_key, &config.secret_key, None);
    MinioClientBuilder::new(base_url)
        .provider(Some(Box::new(creds)))
        .build()
        .context("Failed to build MinIO client for history sink")
}

/// Encode `messages[start..]` as JSONL — one line per message.
///
/// Separate from the upload path so it is unit-testable without a
/// MinIO server.
pub fn encode_jsonl_delta(messages: &[Message], start: usize) -> Result<String> {
    let mut buf = String::new();
    for msg in messages.iter().skip(start) {
        let line =
            serde_json::to_string(msg).context("failed to serialize Message to JSON for sink")?;
        buf.push_str(&line);
        buf.push('\n');
    }
    Ok(buf)
}

/// Overwrite the session's `history.jsonl` object with the full JSONL
/// rendering of `messages`.
///
/// Simplest semantic: PUT the whole file every time. Delta uploads
/// (append semantics over a chunked layout) are a Phase B follow-up —
/// full-file rewrite keeps the sink single-round-trip and easy to
/// reason about for Phase A.
///
/// # Errors
///
/// Returns the underlying MinIO error verbatim; the caller is expected
/// to log-and-swallow (non-fatal sink) unless it is in a test that
/// actually wants upload failure to surface.
pub async fn upload_full_history(
    config: &HistorySinkConfig,
    session_id: &str,
    messages: &[Message],
) -> Result<()> {
    let body = encode_jsonl_delta(messages, 0)?;
    let bytes = body.into_bytes();
    let client = build_client(config)?;
    let content = ObjectContent::from(bytes.clone());
    let key = config.object_key(session_id);
    client
        .put_object_content(&config.bucket, &key, content)
        .send()
        .await
        .with_context(|| {
            format!(
                "failed to PUT s3://{}/{key} ({} bytes)",
                config.bucket,
                bytes.len()
            )
        })?;
    tracing::debug!(
        bucket = %config.bucket,
        key = %key,
        bytes = bytes.len(),
        "history sink upload complete"
    );
    Ok(())
}

/// Stable locator for a single `ResidencyLevel::Pointer` page's
/// backing bytes.
///
/// The Phase B incremental derivation can degrade an `Evidence` page
/// (or any page whose degradation path permits it) to `Pointer` and
/// hand the corresponding [`PointerHandle`] to
/// [`resolve_pointer`] on demand. Keeping the handle pure data makes
/// it serialisable into the page sidecar without dragging the MinIO
/// client along.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::history_sink::PointerHandle;
///
/// let handle = PointerHandle {
///     bucket: "codetether".to_string(),
///     key: "history/abc-123/history.jsonl".to_string(),
///     byte_range: Some((0, 512)),
/// };
/// assert_eq!(handle.byte_range, Some((0, 512)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointerHandle {
    /// S3 bucket name.
    pub bucket: String,
    /// Full object key.
    pub key: String,
    /// Optional half-open `(start, end)` byte range. When `None`, the
    /// full object is fetched.
    #[serde(default)]
    pub byte_range: Option<(u64, u64)>,
}

impl PointerHandle {
    /// Construct a handle that points at the whole history object for
    /// `session_id` under the sink's configured prefix.
    pub fn for_session(config: &HistorySinkConfig, session_id: &str) -> Self {
        Self {
            bucket: config.bucket.clone(),
            key: config.object_key(session_id),
            byte_range: None,
        }
    }
}

/// Fetch the bytes referenced by a [`PointerHandle`] from the MinIO
/// sink.
///
/// Returns a typed [`Fault`] so callers get the same reason-code
/// surface ClawVM §3 prescribes for context-construction failures
/// (*silent* empty returns are forbidden).
///
/// # Errors
///
/// * [`Fault::BackendError`] when the MinIO client build or the GET
///   itself fails.
/// * [`Fault::NoMatch`] when the object is missing but the backend
///   responded cleanly.
///
/// # Range semantics
///
/// When `byte_range` is `Some((start, end))`, the returned slice is
/// truncated to that half-open range within the full object body.
/// Ranges that fall off the end are clamped to `object.len()`.
pub async fn resolve_pointer(
    config: &HistorySinkConfig,
    handle: &PointerHandle,
) -> Result<Vec<u8>, Fault> {
    let client = build_client(config).map_err(|e| Fault::BackendError {
        reason: format!("minio client build failed: {e}"),
    })?;
    let body = client
        .get_object(&handle.bucket, &handle.key)
        .send()
        .await
        .map_err(|e| Fault::BackendError {
            reason: format!("GET s3://{}/{} failed: {e}", handle.bucket, handle.key),
        })?
        .content
        .to_segmented_bytes()
        .await
        .map_err(|e| Fault::BackendError {
            reason: format!("read body for {} failed: {e}", handle.key),
        })?
        .to_bytes();
    if body.is_empty() {
        return Err(Fault::NoMatch);
    }
    let bytes = body.to_vec();
    Ok(match handle.byte_range {
        None => bytes,
        Some((start, end)) => {
            let start = (start as usize).min(bytes.len());
            let end = (end as usize).min(bytes.len()).max(start);
            bytes[start..end].to_vec()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Message, Role};

    #[test]
    fn config_object_key_joins_prefix_and_session_id() {
        let cfg = HistorySinkConfig {
            endpoint: "http://x".to_string(),
            access_key: "a".to_string(),
            secret_key: "s".to_string(),
            bucket: "b".to_string(),
            prefix: "history/".to_string(),
        };
        assert_eq!(
            cfg.object_key("abc-123"),
            "history/abc-123/history.jsonl".to_string()
        );
    }

    #[test]
    fn encode_jsonl_delta_is_line_per_message_and_skippable() {
        let msgs = vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "one".to_string(),
                }],
            },
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: "two".to_string(),
                }],
            },
        ];

        let full = encode_jsonl_delta(&msgs, 0).unwrap();
        assert_eq!(full.lines().count(), 2);
        assert!(full.contains("\"one\""));
        assert!(full.contains("\"two\""));

        let tail = encode_jsonl_delta(&msgs, 1).unwrap();
        assert_eq!(tail.lines().count(), 1);
        assert!(tail.contains("\"two\""));

        let nothing = encode_jsonl_delta(&msgs, 2).unwrap();
        assert!(nothing.is_empty());
    }

    #[test]
    fn pointer_handle_for_session_targets_history_object() {
        let cfg = HistorySinkConfig {
            endpoint: "http://x".to_string(),
            access_key: "a".to_string(),
            secret_key: "s".to_string(),
            bucket: "b".to_string(),
            prefix: "history/".to_string(),
        };
        let handle = PointerHandle::for_session(&cfg, "sess");
        assert_eq!(handle.bucket, "b");
        assert_eq!(handle.key, "history/sess/history.jsonl");
        assert!(handle.byte_range.is_none());
    }

    #[test]
    fn pointer_handle_round_trips_through_serde() {
        let handle = PointerHandle {
            bucket: "b".into(),
            key: "k".into(),
            byte_range: Some((16, 64)),
        };
        let json = serde_json::to_string(&handle).unwrap();
        let back: PointerHandle = serde_json::from_str(&json).unwrap();
        assert_eq!(back, handle);
    }

    #[test]
    fn from_env_returns_none_when_endpoint_unset() {
        // SAFETY: modifying env vars in tests is required for coverage
        // of the disabled-sink code path. `unsafe` is mandated by Rust
        // 2024's tightening of `remove_var`.
        unsafe {
            std::env::remove_var("CODETETHER_HISTORY_S3_ENDPOINT");
        }
        let cfg = HistorySinkConfig::from_env().unwrap();
        assert!(cfg.is_none());
    }
}
