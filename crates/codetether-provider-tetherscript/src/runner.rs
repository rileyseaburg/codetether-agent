//! Thin Rust shell. Loads `.tether`, delegates every trait method.
//!
//! `LoadedPlugin` is `!Send`, so we create a fresh plugin per call
//! inside `spawn_blocking`. The source and credentials are stored.

use anyhow::{Context, Result};
use codetether_provider_core::ModelInfo;

/// Provider backed by a script implementing model listing and completion.
///
/// # Examples
/// ```
/// let provider = codetether_provider_tetherscript::TetherScriptProvider::new("", "", "https://example.test/v1", "example").unwrap();
/// ```
pub struct TetherScriptProvider {
    name: String,
    source: String,
    api_key: String,
    base_url: String,
}

impl Clone for TetherScriptProvider {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            source: self.source.clone(),
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

impl TetherScriptProvider {
    /// Store script source, credential, endpoint, and provider name.
    ///
    /// # Errors
    /// This constructor currently stores values without validating them.
    /// Script load and execution failures are reported by provider calls.
    pub fn new(source: &str, api_key: &str, base_url: &str, name: &str) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            source: source.into(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        })
    }

    pub(crate) fn make_plugin(&self) -> Result<tetherscript::plugin::LoadedPlugin> {
        super::authority::host(&self.base_url, &self.api_key)
            .load_source("p.tether", &self.source)
            .context("load")
    }

    pub(crate) fn name_str(&self) -> &str {
        &self.name
    }

    pub(crate) fn call_list_models(&self) -> Result<Vec<ModelInfo>> {
        self.call_sync("list_models")
            .and_then(|value| super::model_list::parse_models(value, &self.name))
    }
}
