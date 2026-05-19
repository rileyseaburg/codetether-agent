//! Per-task provider registry construction from tenant-scoped claim payloads.

use super::registry::ProviderRegistry;
use crate::secrets::ProviderSecrets;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TenantProviderKeyPayload {
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub providers: HashMap<String, ProviderSecrets>,
}

/// Owned per-task key material. Dropping this value clears the original strings
/// received from the server before provider instances are dropped.
#[derive(Debug)]
pub struct PerTaskProviderKeys {
    payload: TenantProviderKeyPayload,
}

impl PerTaskProviderKeys {
    pub fn from_value(value: &serde_json::Value) -> Result<Option<Self>> {
        if value.is_null() {
            return Ok(None);
        }
        let payload: TenantProviderKeyPayload = serde_json::from_value(value.clone())?;
        if payload.providers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self { payload }))
        }
    }

    pub fn source(&self) -> &str {
        self.payload.source.as_deref().unwrap_or("byok")
    }

    pub fn provider_names(&self) -> Vec<String> {
        let mut names = self.payload.providers.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    pub fn build_registry(&self) -> ProviderRegistry {
        ProviderRegistry::from_provider_secrets_map(&self.payload.providers)
    }
}

impl Drop for PerTaskProviderKeys {
    fn drop(&mut self) {
        for secrets in self.payload.providers.values_mut() {
            if let Some(key) = secrets.api_key.as_mut() {
                unsafe {
                    key.as_bytes_mut().fill(0);
                }
                key.clear();
            }
            if let Some(org) = secrets.organization.as_mut() {
                unsafe {
                    org.as_bytes_mut().fill(0);
                }
                org.clear();
            }
            if let Some(headers) = secrets.headers.as_mut() {
                for value in headers.values_mut() {
                    unsafe {
                        value.as_bytes_mut().fill(0);
                    }
                    value.clear();
                }
                headers.clear();
            }
        }
    }
}
