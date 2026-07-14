//! Canonical identity for one mutating Kubernetes target.

use sha2::{Digest, Sha256};

/// Kubernetes object protected by a distributed mutation lease.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceKey {
    pub namespace: String,
    pub kind: &'static str,
    pub name: String,
}

impl ResourceKey {
    pub fn new(namespace: &str, kind: &'static str, name: &str) -> Self {
        Self {
            namespace: namespace.into(),
            kind,
            name: name.into(),
        }
    }

    pub fn canonical(&self) -> String {
        format!(
            "{}/{}/{}",
            self.namespace,
            self.kind.to_ascii_lowercase(),
            self.name
        )
    }

    pub fn lease_name(&self) -> String {
        let digest = Sha256::digest(self.canonical().as_bytes());
        format!("codetether-mutation-{}", &hex::encode(digest)[..20])
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceKey;

    #[test]
    fn lease_name_is_stable_and_resource_specific() {
        let deployment = ResourceKey::new("prod", "Deployment", "api");
        let service = ResourceKey::new("prod", "Service", "api");
        assert_eq!(deployment.lease_name(), deployment.lease_name());
        assert_ne!(deployment.lease_name(), service.lease_name());
        assert!(deployment.lease_name().len() <= 63);
    }
}
