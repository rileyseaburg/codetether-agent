//! Kubernetes API operations for distributed mutation leases.

use anyhow::{Context, Result};
use k8s_openapi::api::coordination::v1::Lease;
use kube::{
    Api, Client, ResourceExt,
    api::{DeleteParams, PostParams, Preconditions},
};

#[derive(Clone)]
pub struct LeaseRepository {
    api: Api<Lease>,
}

impl LeaseRepository {
    pub fn new(client: Client, namespace: &str) -> Self {
        Self {
            api: Api::namespaced(client, namespace),
        }
    }

    pub async fn create(&self, lease: &Lease) -> Result<Lease> {
        self.api
            .create(&PostParams::default(), lease)
            .await
            .context("create mutation lease")
    }

    pub async fn get(&self, name: &str) -> Result<Lease> {
        self.api.get(name).await.context("read mutation lease")
    }

    pub async fn replace(&self, lease: &Lease) -> Result<Lease> {
        self.api
            .replace(&lease.name_any(), &PostParams::default(), lease)
            .await
            .context("replace mutation lease")
    }

    pub async fn release(&self, lease: &Lease) -> Result<()> {
        let params = DeleteParams {
            preconditions: Some(Preconditions {
                resource_version: lease.metadata.resource_version.clone(),
                uid: lease.metadata.uid.clone(),
            }),
            ..DeleteParams::default()
        };
        self.api
            .delete(&lease.name_any(), &params)
            .await
            .context("release mutation lease")?;
        Ok(())
    }
}
