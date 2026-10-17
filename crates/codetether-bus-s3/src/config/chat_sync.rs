//! chat sync for the S3 bus adapter.
use super::*;
impl BusS3SinkConfig {
    pub(super) fn from_chat_sync() -> Option<Self> {
        let endpoint = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT");
        let access_key = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY");
        let secret_key = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY");

        if let (Some(ep), Some(ak), Some(sk)) =
            (endpoint.clone(), access_key.clone(), secret_key.clone())
        {
            info!("Bus S3 sink using chat-sync env vars");
            return Some(Self {
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

        None
    }
}
