//! Single global manager registration and convenience access.

use crate::SecretsManager;
mod initialize;
mod read;
mod write;

/// Global secrets manager instance
static SECRETS_MANAGER: tokio::sync::OnceCell<SecretsManager> = tokio::sync::OnceCell::const_new();
pub use initialize::{init_from_manager, init_secrets_manager, secrets_manager};
pub use read::{get_api_key, get_provider_secrets, has_api_key};
pub use write::{set_provider_secrets, verify_reachable};
