//! DeepSeek V4 provider — OpenAI-compatible with thinking-mode support.
//!
//! DeepSeek V4 returns `reasoning_content` alongside `content`.  When tool
//! calls are present, the API **requires** that `reasoning_content` is
//! round-tripped in subsequent requests (otherwise HTTP 400).  This provider
//! stores reasoning as [`ContentPart::Thinking`] and re-emits it.

mod body;
mod complete;
mod convert;
mod convert_tools;
mod error;
mod models;
mod parse_response;
mod provider_impl;
mod response;
mod stream;

use anyhow::Result;
use reqwest::Client;

pub struct DeepSeekProvider {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
}

impl DeepSeekProvider {
    /// Create a new provider with a custom base URL.
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            api_key,
            base_url,
        })
    }
}
