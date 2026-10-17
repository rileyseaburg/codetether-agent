//! vault for the S3 bus adapter.
use super::*;
impl BusS3SinkConfig {
    pub(super) async fn from_vault() -> Option<Self> {
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
        }

        None
    }
}
