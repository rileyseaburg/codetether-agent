//! Ownership-safe lease renewal and release.

use anyhow::Result;
use chrono::Utc;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;

use super::decision::owns;
use super::{manifest, repository::LeaseRepository};

pub async fn renew(repo: &LeaseRepository, name: &str, holder: &str) -> Result<bool> {
    let mut current = repo.get(name).await?;
    if !owns(&manifest::state(&current), holder) {
        return Ok(false);
    }
    let now = k8s_openapi::jiff::Timestamp::from_second(Utc::now().timestamp())?;
    if let Some(spec) = current.spec.as_mut() {
        spec.renew_time = Some(MicroTime(now));
    }
    repo.replace(&current).await?;
    Ok(true)
}

pub async fn release(repo: &LeaseRepository, name: &str, holder: &str) -> Result<bool> {
    let current = match repo.get(name).await {
        Ok(lease) => lease,
        Err(error) if is_not_found(&error) => return Ok(false),
        Err(error) => return Err(error),
    };
    if !owns(&manifest::state(&current), holder) {
        return Ok(false);
    }
    repo.release(&current).await?;
    Ok(true)
}

fn is_not_found(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<kube::Error>()
        .is_some_and(|error| matches!(error, kube::Error::Api(response) if response.code == 404))
}
