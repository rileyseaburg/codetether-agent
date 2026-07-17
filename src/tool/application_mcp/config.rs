//! Trusted application MCP configuration sourced from process environment.

use anyhow::{Result, anyhow};

pub(super) struct Config {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) endpoint: reqwest::Url,
    pub(super) token: String,
}

pub(super) fn load() -> Result<Option<Config>> {
    let Some(endpoint) =
        value("CODETETHER_APPLICATION_MCP_URL").or_else(|| value("CODETETHER_FASTIFY_MCP_URL"))
    else {
        return Ok(None);
    };
    let id =
        value("CODETETHER_APPLICATION_MCP_ID").unwrap_or_else(|| "mcp__application".to_string());
    if !valid_id(&id) {
        return Err(anyhow!("invalid CODETETHER_APPLICATION_MCP_ID"));
    }
    let name =
        value("CODETETHER_APPLICATION_MCP_NAME").unwrap_or_else(|| "application".to_string());
    let token = value("CODETETHER_APPLICATION_MCP_TOKEN")
        .or_else(|| value("CODETETHER_OPENAI_API_KEY"))
        .ok_or_else(|| anyhow!("application MCP bearer token is missing"))?;
    let endpoint = reqwest::Url::parse(&endpoint)?;
    if !matches!(endpoint.scheme(), "http" | "https") {
        return Err(anyhow!("application MCP endpoint must use HTTP or HTTPS"));
    }
    Ok(Some(Config {
        id,
        name,
        endpoint,
        token,
    }))
}

fn value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn valid_id(id: &str) -> bool {
    id.starts_with("mcp__")
        && id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
}
