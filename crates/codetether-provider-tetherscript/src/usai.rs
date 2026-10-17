//! USAI provider defaults and construction from the shared example script.
use std::sync::Arc;

use anyhow::Result;

use codetether_provider_core::Provider;

use super::TetherScriptProvider;

const SOURCE: &str = include_str!("../../../examples/tetherscript/usai_chat.tether");
const BASE_URL: &str = "https://api.gsa.usai.gov/api/v1";
const NAME: &str = "usai";

/// Construct the USAI script provider with an optional endpoint override.
///
/// # Arguments
/// * `api_key` - Credential bound to the provider capability.
/// * `base_url` - Optional API base URL, otherwise the existing USAI default.
/// # Returns
/// A provider storing the embedded script and credentials.
/// # Errors
/// Propagates construction errors from the script provider.
pub fn new(api_key: &str, base_url: Option<&str>) -> Result<TetherScriptProvider> {
    TetherScriptProvider::new(SOURCE, api_key, base_url.unwrap_or(BASE_URL), NAME)
}

/// Construct the USAI provider behind the common Provider trait.
///
/// # Errors
/// Propagates provider construction errors.
pub fn provider(api_key: &str, base_url: Option<&str>) -> Result<Arc<dyn Provider>> {
    Ok(Arc::new(new(api_key, base_url)?))
}

/// Return the existing USAI API endpoint default.
pub fn default_base_url() -> &'static str {
    BASE_URL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_usai_models_from_tetherscript() {
        let provider = new("test-key", Some("http://localhost:8080/api/v1")).unwrap();
        let models = provider.call_list_models().unwrap();

        assert_eq!(models[0].provider, "usai");
        assert!(models.iter().any(|model| model.id == "gemini-2.5-flash"));
        assert!(models.iter().any(|model| model.id == "claude_4_5_sonnet"));
    }

    #[test]
    fn exposes_default_base_url() {
        assert_eq!(default_base_url(), "https://api.gsa.usai.gov/api/v1");
    }
}
