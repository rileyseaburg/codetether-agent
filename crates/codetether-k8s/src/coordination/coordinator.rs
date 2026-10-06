//! Public facade for acquiring mutation ownership.

use anyhow::Result;
use kube::Client;

use super::claim::MutationClaim;
use super::guard::LeaseGuard;
use super::repository::LeaseRepository;
use super::resource::ResourceKey;
use super::{acquire, config, identity};

/// Coordinates exclusive ownership of Kubernetes mutations.
#[derive(Clone, Debug, Default)]
pub struct MutationCoordinator;

impl MutationCoordinator {
    /// Acquires a renewable lease for one Kubernetes resource mutation.
    pub async fn acquire(
        &self,
        client: Client,
        namespace: &str,
        kind: &'static str,
        name: &str,
        operation: &str,
    ) -> Result<MutationClaim> {
        let key = ResourceKey::new(namespace, kind, name);
        let holder = identity::new_holder(operation);
        let ttl = config::lease_seconds();
        let repo = LeaseRepository::new(client, namespace);
        acquire::acquire(&repo, &key, &holder, ttl).await?;
        let guard = LeaseGuard::start(repo, key.lease_name(), holder.clone(), ttl);
        Ok(MutationClaim::new(
            guard,
            holder,
            operation.into(),
            identity::field_manager(),
        ))
    }
}
