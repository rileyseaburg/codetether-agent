//! Atomic create-or-takeover protocol for mutation leases.

use anyhow::Result;
use chrono::Utc;
use k8s_openapi::api::coordination::v1::Lease;
use kube::Error;

use super::decision::{LeaseDecision, decide};
use super::error::LeaseConflict;
use super::{manifest, repository::LeaseRepository, resource::ResourceKey};

pub async fn acquire(
    repo: &LeaseRepository,
    key: &ResourceKey,
    holder: &str,
    ttl: i32,
) -> Result<Lease> {
    let desired = manifest::build(key, holder, ttl, Utc::now());
    match repo.create(&desired).await {
        Ok(lease) => return Ok(lease),
        Err(error) if is_conflict(&error) => {}
        Err(error) => return Err(error),
    }
    let current = repo.get(&key.lease_name()).await?;
    match decide(Some(&manifest::state(&current)), holder, Utc::now()) {
        LeaseDecision::Conflict { holder, expires_at } => Err(LeaseConflict {
            resource: key.canonical(),
            holder,
            expires_at,
        }
        .into()),
        LeaseDecision::Renew | LeaseDecision::TakeOver => {
            let mut replacement = desired;
            replacement.metadata.resource_version = current.metadata.resource_version;
            replacement.metadata.uid = current.metadata.uid;
            if let Some(spec) = replacement.spec.as_mut() {
                spec.lease_transitions = current
                    .spec
                    .and_then(|s| s.lease_transitions)
                    .map(|n| n + 1);
            }
            repo.replace(&replacement).await
        }
        LeaseDecision::Acquire => unreachable!("existing lease was loaded"),
    }
}

fn is_conflict(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<Error>()
        .is_some_and(|error| matches!(error, Error::Api(response) if response.code == 409))
}
