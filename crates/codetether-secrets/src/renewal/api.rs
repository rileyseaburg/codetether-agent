//! Bound Vault requests and retain only non-secret lease metadata.

use super::failure::{Failure, classify};
use std::time::Duration;
use vaultrs::{client::VaultClient, token};

#[derive(Clone, Copy)]
pub(super) struct Lease {
    pub ttl: u64,
    pub renewable: bool,
}

pub(super) async fn lookup(client: &VaultClient) -> Result<Lease, Failure> {
    let result = tokio::time::timeout(Duration::from_secs(10), token::lookup_self(client))
        .await
        .map_err(|_| Failure::Timeout)?
        .map_err(classify)?;
    Ok(Lease {
        ttl: result.ttl,
        renewable: result.renewable == Some(true) && result.num_uses == 0,
    })
}

pub(super) async fn discover(client: &VaultClient) -> Result<Lease, Failure> {
    match lookup(client).await {
        Ok(lease) => Ok(lease),
        Err(error) => {
            tracing::warn!(failure = ?error, "Vault token lookup failed; attempting renew-self directly");
            renew(client).await
        }
    }
}

pub(super) async fn renew(client: &VaultClient) -> Result<Lease, Failure> {
    let result = tokio::time::timeout(Duration::from_secs(10), token::renew_self(client, None))
        .await
        .map_err(|_| Failure::Timeout)?
        .map_err(classify)?;
    Ok(Lease {
        ttl: result.lease_duration,
        renewable: result.renewable,
    })
}
