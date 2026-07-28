//! BrowserPage permission and geolocation APIs.

use super::PermissionState;
use super::{BrowserPermission, GeolocationEmulation, GeolocationError, GeolocationPosition};
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::security::Origin;

impl BrowserPage {
    /// Grant one permission for an origin on this page.
    pub fn grant_permission(
        &mut self,
        origin_url: impl AsRef<str>,
        permission: BrowserPermission,
    ) -> Origin {
        self.set_permission(origin_url, permission, PermissionState::Granted)
    }

    /// Deny one permission for an origin on this page.
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

    /// Return this page's geolocation emulation outcome.
    pub fn geolocation(&self) -> &GeolocationEmulation {
        &self.geolocation
    }

    /// Configure this page to return a geolocation position.
    pub fn set_geolocation(&mut self, position: GeolocationPosition) {
        self.geolocation = GeolocationEmulation::Position(position);
    }

    /// Configure this page to return a geolocation error.
    pub fn set_geolocation_error(&mut self, error: GeolocationError) {
        self.geolocation = GeolocationEmulation::Error(error);
    }

    /// Clear this page's configured geolocation outcome.
    pub fn clear_geolocation(&mut self) {
        self.geolocation = GeolocationEmulation::Unavailable;
    }
}
