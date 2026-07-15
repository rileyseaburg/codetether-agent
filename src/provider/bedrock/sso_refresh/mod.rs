//! Browser-free silent refresh of a stored Bedrock API key.
//!
//! When a Bedrock bearer token nears expiry, this module re-mints it without
//! any device-code prompt by reusing the SSO refresh material saved alongside
//! the provider secret (OIDC `refresh_token` grant + `GetRoleCredentials`).
//!
//! Two entry points share the same machinery:
//! - [`ensure_fresh`] — proactive startup assessment (called by the provider
//!   registry before building the Bedrock provider).
//! - [`refresh_now`] — explicit CLI `--refresh`.

mod auto;
mod exported;
mod oidc_refresh;
mod refresh_flow;
mod role_creds;
mod role_creds_types;
mod save;
mod staleness;
mod stored;
mod vault_expiry;

pub use auto::ensure_fresh;
pub(crate) use auto::refresh_now;
