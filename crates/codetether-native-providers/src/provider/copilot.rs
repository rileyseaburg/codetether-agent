//! Copilot provider bound to the CodeTether application identity.
pub use codetether_provider_copilot::{enterprise_base_url, normalize_enterprise_domain};
mod identity;
/// Copilot provider using the application's package version in its User-Agent.
/// # Examples
/// ```no_run
/// let _provider = codetether_native_providers::copilot::CopilotProvider::new("token".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub type CopilotProvider = codetether_provider_copilot::application::CopilotProvider<identity::App>;

pub(in crate::provider) use identity::App as ApplicationIdentity;
