//! GitHub Copilot application adapter.
//!
//! [CopilotProvider] supplies the application version in the User-Agent and
//! delegates catalog discovery and completions to the workspace provider crate.
//! Construct it with [CopilotProvider::new] or [CopilotProvider::enterprise].
pub use codetether_provider_copilot::{enterprise_base_url, normalize_enterprise_domain};
mod construct;
mod delegate;
/// Copilot provider carrying the CodeTether application identity.
///
/// # Examples
/// ```no_run
/// let provider = codetether_agent::provider::copilot::CopilotProvider::new("token".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct CopilotProvider(codetether_provider_copilot::CopilotProvider);
