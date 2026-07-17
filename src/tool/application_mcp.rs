//! Environment-configured, authenticated application MCP connection.

#[path = "application_mcp/config.rs"]
mod config;
#[path = "application_mcp/invocation.rs"]
mod invocation;
#[path = "application_mcp/request.rs"]
mod request;
#[path = "application_mcp/schema.rs"]
mod schema;
#[path = "application_mcp/tool_impl.rs"]
mod tool_impl;

use crate::tool::ToolRegistry;
use std::sync::Arc;

pub(super) struct ApplicationMcpTool {
    id: String,
    name: String,
    description: String,
    endpoint: reqwest::Url,
    token: String,
    client: reqwest::Client,
}

impl ApplicationMcpTool {
    fn new(config: config::Config) -> anyhow::Result<Self> {
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

pub(crate) fn register(registry: &mut ToolRegistry) {
    match config::load().and_then(|value| value.map(ApplicationMcpTool::new).transpose()) {
        Ok(Some(tool)) => registry.register(Arc::new(tool)),
        Ok(None) => {}
        Err(error) => tracing::warn!(error = %error, "Application MCP connection disabled"),
    }
}
