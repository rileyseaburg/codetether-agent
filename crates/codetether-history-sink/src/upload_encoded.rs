//! Upload an already encoded session history object.

use anyhow::{Context, Result};
use minio::s3::builders::ObjectContent;

use super::HistorySinkConfig;

/// Upload pre-encoded history bytes at the existing session object key.
/// # Arguments
/// `config` provides the endpoint and credentials; `session_id` selects the
/// object key; `bytes` are the full body to PUT.
/// # Returns
/// Unit when the backend acknowledges the upload.
/// # Errors
/// Propagates client construction and backend upload errors with object context.
/// # Examples
/// ```rust,no_run
/// # async fn example(config: &codetether_history_sink::HistorySinkConfig) -> anyhow::Result<()> {
/// codetether_history_sink::upload_encoded_history(config, "session", b"{}\n".to_vec()).await?;
/// # Ok(()) }
/// ```
pub async fn encoded(config: &HistorySinkConfig, session_id: &str, bytes: Vec<u8>) -> Result<()> {
    let byte_len = bytes.len();
    let client = crate::client::build_client(config)?;
    let key = config.object_key(session_id);
    client
        .put_object_content(&config.bucket, &key, ObjectContent::from(bytes))?
        .build()
        .send()
        .await
        .with_context(|| {
            format!(
                "failed to PUT s3://{}/{key} ({} bytes)",
                config.bucket, byte_len
            )
        })?;
    tracing::debug!(target: "codetether_agent::session::history_sink::upload",
        bucket = %config.bucket,
        key = %key,
        bytes = byte_len,
        "history sink upload complete"
    );
    Ok(())
}
