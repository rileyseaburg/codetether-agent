//! Mutation helpers for the permission store.

use super::{BrowserPermission, PermissionGrant, PermissionState, PermissionStore};
use crate::browser_agent::security::Origin;

impl PermissionStore {
    /// Set the decision for one origin and permission.
    pub fn set(&mut self, origin: &Origin, permission: BrowserPermission, state: PermissionState) {
        let key = origin.serialized();
        if let Some(grant) = self
            .grants
            .iter_mut()
            .find(|grant| grant.origin == key && grant.permission == permission)
        {
            grant.state = state;
        } else {
            self.grants
                .push(PermissionGrant::new(key, permission, state));
        }
        self.sort();
    }

    /// Clear all permission decisions for one origin.
    pub fn clear_origin(&mut self, origin: &Origin) {
        let key = origin.serialized();
        self.grants.retain(|grant| grant.origin != key);
    }

    /// Replace every stored grant.
    pub fn replace_all(&mut self, grants: Vec<PermissionGrant>) {
        self.grants = grants;
        self.sort();
    }

    fn sort(&mut self) {
        self.grants
            .sort_by_key(|grant| (grant.origin.clone(), grant.permission));
    }
}
