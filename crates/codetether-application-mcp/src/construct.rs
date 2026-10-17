//! Construct the authenticated connection and its bounded HTTP client.
use super::{ApplicationMcpTool, config};

impl ApplicationMcpTool {
    pub(super) fn new(config: config::Config) -> anyhow::Result<Self> {
        let description = format!(
            "Authenticated MCP connection to {}. Use list_tools before call_tool; this connection is available to you.",
            config.name
        );
        Ok(Self {
            id: config.id,
            name: config.name,
            description,
            endpoint: config.endpoint,
            token: config.token,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()?,
        })
    }
}
