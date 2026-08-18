//! Upload an already encoded session history object.

use anyhow::{Context, Result};
use minio::s3::builders::ObjectContent;

use super::HistorySinkConfig;

pub(crate) async fn encoded(
    config: &HistorySinkConfig,
    session_id: &str,
    bytes: Vec<u8>,
) -> Result<()> {
    let byte_len = bytes.len();
    let client = super::build_client(config)?;
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
    tracing::debug!(
        bucket = %config.bucket,
        key = %key,
        bytes = byte_len,
        "history sink upload complete"
    );
    Ok(())
}
