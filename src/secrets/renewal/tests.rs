//! Mocked HTTP Vault tests exercise real lookup-self/renew-self SDK requests.

mod counts;
mod fixture;
mod handlers;
mod lifecycle;
mod payloads;
mod policies;

use crate::secrets::{SecretsManager, VaultConfig};
use std::{sync::atomic::Ordering, time::Duration};

async fn manager(server: &fixture::Fixture) -> SecretsManager {
    SecretsManager::new(&VaultConfig {
        address: server.address.clone(),
        token: "fixture-token-not-a-secret".into(),
        mount: None,
        path: None,
    })
    .await
    .unwrap()
}

async fn wait_for(counter: &std::sync::atomic::AtomicUsize, count: usize) {
    tokio::time::timeout(Duration::from_secs(5), async {
        while counter.load(Ordering::SeqCst) < count {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("expected Vault request was not observed");
}
