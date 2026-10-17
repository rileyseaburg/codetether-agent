//! Tenant provider-key payloads and selected owned-string cleanup.
//!
//! [TenantProviderKeyPayload] carries the claim data. The application decides
//! when to clear key strings as part of its task-owner lifecycle.
use crate::ProviderSecrets;
use serde::Deserialize;
use std::collections::HashMap;

/// Tenant-scoped provider credentials decoded from a task claim.
///
/// # Examples
/// ```
/// let payload = codetether_secrets::TenantProviderKeyPayload::default();
/// assert!(payload.provider_names().is_empty());
/// ```
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TenantProviderKeyPayload {
    #[serde(default)]
    /// Optional claim source; accessors use the existing BYOK fallback.
    pub source: Option<String>,
    #[serde(default)]
    /// Provider-key material indexed by provider identifier.
    pub providers: HashMap<String, ProviderSecrets>,
}
mod access;
mod clear;
mod parse;
