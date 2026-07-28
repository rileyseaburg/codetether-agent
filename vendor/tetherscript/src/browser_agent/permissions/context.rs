//! BrowserContext permission and geolocation APIs.

use super::PermissionState;
use super::{BrowserPermission, GeolocationEmulation, GeolocationError, GeolocationPosition};
use crate::browser_agent::context::BrowserContext;
use crate::browser_agent::security::Origin;

impl BrowserContext {
    /// Grant one permission for an origin in this context.
    pub fn grant_permission(
        &mut self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
    ) -> Origin {
        self.set_permission(origin_url, permission, PermissionState::Granted)
    }

    /// Deny one permission for an origin in this context.
    pub fn deny_permission(
        &mut self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
    ) -> Origin {
        self.set_permission(origin_url, permission, PermissionState::Denied)
    }

    /// Return the configured permission state for an origin.
    pub fn permission_state(
        &self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
    ) -> PermissionState {
        self.permissions
            .get(&Origin::parse(origin_url.as_ref()), permission)
    }

    /// Return this context's default geolocation emulation outcome.
    pub fn geolocation(&self) -> &GeolocationEmulation {
        &self.geolocation
    }

    /// Configure context default geolocation for existing and new pages.
    pub fn set_geolocation(&mut self, position: GeolocationPosition) {
        self.geolocation = GeolocationEmulation::Position(position);
        self.apply_geolocation();
    }

    /// Configure context default geolocation error.
    pub fn set_geolocation_error(&mut self, error: GeolocationError) {
        self.geolocation = GeolocationEmulation::Error(error);
        self.apply_geolocation();
    }

    /// Clear context default geolocation.
    pub fn clear_geolocation(&mut self) {
        self.geolocation = GeolocationEmulation::Unavailable;
        self.apply_geolocation();
    }
}
