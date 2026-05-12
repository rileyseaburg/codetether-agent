//! S3/MinIO config for trace persistence.

/// Decoupled from the host crate's `S3Config`.
#[derive(Debug, Clone)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub prefix: String,
    pub secure: bool,
    pub ignore_cert: bool,
}
impl S3Config {
    /// Build from standard environment variables.
    pub async fn from_env_or_vault() -> anyhow::Result<Self> {
        Ok(Self {
            endpoint: std::env::var("MINIO_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".into()),
            access_key: std::env::var("MINIO_ACCESS_KEY")?,
            secret_key: std::env::var("MINIO_SECRET_KEY")?,
            bucket: std::env::var("MINIO_RLM_BUCKET")
                .unwrap_or_else(|_| "codetether-rlm-traces".into()),
            prefix: std::env::var("MINIO_RLM_PREFIX")
                .unwrap_or_else(|_| "training/".into()),
            secure: std::env::var("MINIO_SECURE").as_deref() == Ok("true"),
            ignore_cert: std::env::var("MINIO_IGNORE_CERT").as_deref() == Ok("true"),
        })
    }
}