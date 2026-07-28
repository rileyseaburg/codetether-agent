//! Read helpers for the permission store.

use super::{BrowserPermission, PermissionGrant, PermissionState, PermissionStore};
use crate::browser_agent::permissions::kind::ALL_PERMISSIONS;
use crate::browser_agent::security::Origin;

impl PermissionStore {
    /// Return true when no permission decisions are configured.
    pub fn is_empty(&self) -> bool {
        self.grants.is_empty()
    }

    /// Return the decision for an origin and permission.
    pub fn get(&self, origin: &Origin, permission: BrowserPermission) -> PermissionState {
        let key = origin.serialized();
        self.grants
            .iter()
            .find(|grant| grant.origin == key && grant.permission == permission)
            .map(|grant| grant.state)
            .unwrap_or_default()
    }

    /// Return grants in deterministic order.
    pub fn grants(&self) -> Vec<PermissionGrant> {
        self.grants.clone()
    }

    pub(crate) fn bridge_states(
        &self,
        origin: &Origin,
    ) -> Vec<(BrowserPermission, PermissionState)> {
        ALL_PERMISSIONS
            .iter()
            .map(|permission| (*permission, self.get(origin, *permission)))
            .collect()
    }
}
