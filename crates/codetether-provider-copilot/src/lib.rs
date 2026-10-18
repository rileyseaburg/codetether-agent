//! GitHub Copilot completions and model discovery.
//!
//! Construct [CopilotProvider] with a token and caller-supplied User-Agent, then
//! use [Provider]. Request encoding, discovery, catalog metadata, transport,
//! and response interpretation are independently owned modules.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, Message, ModelInfo, Provider, Role,
    StreamChunk, ToolDefinition,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
mod catalog;
mod construct;
mod discovery;
mod enterprise;
mod pricing;
mod provider;
mod request;
mod response;
mod transport;
mod wire;
pub use enterprise::{enterprise_base_url, normalize_enterprise_domain};
use wire::*;
const DEFAULT_BASE_URL: &str = "https://api.githubcopilot.com";
const COPILOT_PROVIDER: &str = "github-copilot";
const COPILOT_ENTERPRISE_PROVIDER: &str = "github-copilot-enterprise";
/// Copilot provider configured with endpoint credentials and caller identity.
///
/// # Examples
/// ```no_run
/// let provider = codetether_provider_copilot::CopilotProvider::new(
///     "token".into(), "example-client/1.0".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct CopilotProvider {
    client: Client,
    user_agent: String,
    token: String,
    base_url: String,
    provider_name: String,
}
#[cfg(test)]
mod tests;
mod trait_impl;

/// Caller-identity wrapper retaining application User-Agent behavior.
pub mod application;
