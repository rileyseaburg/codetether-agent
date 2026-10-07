//! URL-bound credential state; intentionally does not implement Debug.

use serde::{Deserialize, Serialize};

/// Saved settings for one active Vault server; tokens never appear in status output.
#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    pub address: String,
    #[serde(default)]
    pub token: Option<String>,
}

impl Profile {
    /// Profile for `address` with no stored token.
    pub fn without_token(address: String) -> Self {
        Self {
            address,
            token: None,
        }
    }
}
