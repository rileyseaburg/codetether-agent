//! Browser page route-table APIs.

use crate::browser_agent::page::BrowserPage;

use super::{NetworkLogEntry, NetworkRoute, RouteAction, RouteId, RouteRule};

impl BrowserPage {
    /// Register a network route for this page.
    pub fn route(&mut self, rule: RouteRule, action: RouteAction) -> RouteId {
        self.network_routes.borrow_mut().add(rule, action)
    }

    /// Remove a page network route by id.
    pub fn unroute(&mut self, id: RouteId) -> Option<NetworkRoute> {
        self.network_routes.borrow_mut().remove(id)
    }

    /// Return registered page routes in insertion order.
    pub fn routes(&self) -> Vec<NetworkRoute> {
        self.network_routes.borrow().routes().to_vec()
    }

    /// Return deterministic route decisions made for this page.
    pub fn route_log(&self) -> Vec<NetworkLogEntry> {
        self.network_routes.borrow().log().to_vec()
    }
}
