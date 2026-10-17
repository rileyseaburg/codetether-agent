//! client for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    /// Create sink from configuration
    pub async fn from_config(bus: Arc<AgentBus>, config: BusS3SinkConfig) -> Result<Self> {
        let endpoint = normalize_endpoint(&config.endpoint, config.secure);

        let base_url: BaseUrl = endpoint.parse().context("Invalid MinIO endpoint URL")?;

        let static_provider = StaticProvider::new(&config.access_key, &config.secret_key, None);

        let client = MinioClientBuilder::new(base_url)
            .provider(Some(static_provider))
            .ignore_cert_check(Some(config.ignore_cert))
            .build()?;

        let rx = bus.subscribe();

        Ok(Self {
            _bus: bus,
            client,
            config,
            rx,
        })
    }
}
