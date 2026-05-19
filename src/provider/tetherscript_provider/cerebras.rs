use std::sync::Arc;

use anyhow::Result;

use crate::provider::traits::Provider;

use super::TetherScriptProvider;

const SOURCE: &str = include_str!("../../../examples/tetherscript/cerebras_chat.tether");
const BASE_URL: &str = "https://api.cerebras.ai/v1";
const NAME: &str = "cerebras";

pub fn new(api_key: &str, base_url: Option<&str>) -> Result<TetherScriptProvider> {
    TetherScriptProvider::new(SOURCE, api_key, base_url.unwrap_or(BASE_URL), NAME)
}

pub fn provider(api_key: &str, base_url: Option<&str>) -> Result<Arc<dyn Provider>> {
    Ok(Arc::new(new(api_key, base_url)?))
}

pub fn default_base_url() -> &'static str {
    BASE_URL
}
