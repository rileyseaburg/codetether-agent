//! Task-owned credentials bound to an application's provider factory and registry.
//!
//! [BoundTaskKeys] retains the shared credential owner, including its drop-time
//! clearing, and converts constructed registries into the application's type.
use crate::{PerTaskProviderKeys, ProviderRegistry};
use std::marker::PhantomData;
mod access;
mod construct;
mod registry;

/// Task credentials whose registry type and provider factory are fixed by the application.
/// # Examples
/// ```
/// # fn example<H: codetether_provider_factory::Host>() -> anyhow::Result<()> {
/// let keys = codetether_provider_registry::BoundTaskKeys::<H,
///     codetether_provider_registry::ProviderRegistry>::from_value(&serde_json::Value::Null)?;
/// assert!(keys.is_none());
/// # Ok(()) }
/// ```
pub struct BoundTaskKeys<H, R> {
    inner: PerTaskProviderKeys,
    binding: PhantomData<fn() -> (H, R)>,
}

impl<H, R> std::fmt::Debug for BoundTaskKeys<H, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}
