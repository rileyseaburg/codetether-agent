//! Disconnected manager defaults.

use crate::SecretsManager;
use parking_lot::RwLock as ClientRwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

impl Default for SecretsManager {
    fn default() -> Self {
        Self {
            client: Arc::new(ClientRwLock::new(None)),
            cache: Arc::new(RwLock::new(HashMap::new())),
            mount: "secret".to_string(),
            path: "codetether/providers".to_string(),
            k8s_auth: None,
            renewal: Arc::default(),
        }
    }
}
