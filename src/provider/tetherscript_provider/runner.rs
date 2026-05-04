//! Thin Rust shell. Loads `.tether`, delegates every trait method.
//!
//! `LoadedPlugin` is `!Send`, so we create a fresh plugin per call
//! inside `spawn_blocking`. The source and credentials are stored.

use super::super::ModelInfo;
use anyhow::{Context, Result};

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
    pub fn new(source: &str, api_key: &str, base_url: &str, name: &str) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            source: source.into(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        })
    }

    pub(crate) fn make_plugin(&self) -> Result<tetherscript::plugin::LoadedPlugin> {
        use tetherscript::plugin::{PluginHost, TetherScriptAuthority};
        use tetherscript::provider_cap::ProviderAuthority;
        let mut h = PluginHost::new();
        h.grant("tetherscript", TetherScriptAuthority::new());
        let pa = ProviderAuthority::new(&self.base_url);
        let pa = ProviderAuthority::with_bound_header(
            pa,
            "Authorization",
            &format!("Bearer {}", self.api_key),
        );
        h.grant("provider", pa);
        h.load_source("p.tether", &self.source).context("load")
    }

    pub(crate) fn name_str(&self) -> &str {
        &self.name
    }

    pub(crate) fn call_list_models(&self) -> Result<Vec<ModelInfo>> {
        self.call_sync("list_models")
            .and_then(|v| serde_json::from_value(v).map_err(Into::into))
    }
}
