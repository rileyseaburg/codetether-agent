//! Shared credential state, refresh coordination, and persistence.
//!
//! [CredentialManager] borrows provider state while coordinating refresh;
//! [VaultCredentialStore] persists rotations and [RefreshLock] serializes
//! refreshes across local processes.
use crate::{CodexOAuth, OAuthCredentials};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
mod lock;
mod persist;
mod policy;
mod refresh;
mod rotate;
mod state;
mod store;
mod sync;
pub use lock::RefreshLock;
pub use policy::refresh_required;
pub use rotate::merge_refreshed_credentials;
pub use state::OAuthCredentialState;
pub use store::{VaultCredentialStore, credentials, updated_vault_secret};
pub use sync::adopt_shared_credentials;
/// Borrowed coordination context sharing the provider's credential slot.
///
/// # Examples
/// See [CredentialManager::new] for the borrowed inputs.
pub struct CredentialManager<'a> {
    client: &'a reqwest::Client,
    chatgpt_account_id: &'a Option<String>,
    stored_credentials: &'a Option<Arc<RwLock<OAuthCredentialState>>>,
    credential_store: &'a Option<VaultCredentialStore>,
}
impl<'a> CredentialManager<'a> {
    /// Borrow a client, account identity, credential slot, and optional Vault store.
    pub fn new(
        client: &'a reqwest::Client,
        chatgpt_account_id: &'a Option<String>,
        stored_credentials: &'a Option<Arc<RwLock<OAuthCredentialState>>>,
        credential_store: &'a Option<VaultCredentialStore>,
    ) -> Self {
        Self {
            client,
            chatgpt_account_id,
            stored_credentials,
            credential_store,
        }
    }
}
