//! from sources for the S3 bus adapter.
use super::*;
impl BusS3SinkConfig {
    /// Create config by trying multiple credential sources in order:
    ///
    /// 1. Bus-specific env vars (`MINIO_ENDPOINT`, `CODETETHER_BUS_S3_ENDPOINT`)
    /// 2. Chat-sync env vars (`CODETETHER_CHAT_SYNC_MINIO_ENDPOINT`)
    /// 3. Vault secrets at `secret/codetether/providers/chat-sync-minio`
    pub async fn from_env_or_vault() -> Result<Self> {
        if let Ok(cfg) = Self::from_env() {
            return Ok(cfg);
        }
        if let Some(cfg) = Self::from_chat_sync() {
            return Ok(cfg);
        }
        if let Some(cfg) = Self::from_vault().await {
            return Ok(cfg);
        }
        anyhow::bail!(
            "No MinIO credentials found. Set MINIO_ENDPOINT/MINIO_ACCESS_KEY/MINIO_SECRET_KEY, \
             CODETETHER_CHAT_SYNC_MINIO_* env vars, or configure chat-sync-minio in Vault."
        )
    }
}
