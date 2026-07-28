//! Deterministic network route table.

use super::{NetworkLogEntry, NetworkRoute, RouteAction, RouteId, RouteRequest, RouteRule};

/// Ordered collection of route rules for one page or browser context.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RouteAction, RoutePattern, RouteRequest, RouteRule, RouteTable};
///
/// let mut table = RouteTable::default();
/// table.add(RouteRule::new(RoutePattern::glob("**/api/*")), RouteAction::abort("blocked"));
/// let action = table.handle(RouteRequest::new("GET", "https://x.test/api/users"));
/// assert_eq!(action, RouteAction::abort("blocked"));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteTable {
    routes: Vec<NetworkRoute>,
    log: Vec<NetworkLogEntry>,
    next_id: u64,
    next_sequence: u64,
}

impl RouteTable {
    /// Registers a route and returns its stable id.
    pub fn add(&mut self, rule: RouteRule, action: RouteAction) -> RouteId {
        let id = RouteId(self.next_id);
        self.next_id += 1;
        self.routes.push(NetworkRoute { id, rule, action });
        id
    }

    /// Removes a route by id.
    pub fn remove(&mut self, id: RouteId) -> Option<NetworkRoute> {
        let index = self.routes.iter().position(|route| route.id == id)?;
        Some(self.routes.remove(index))
    }

    /// Returns the newest matching route for `request`.
    pub fn match_request(&self, request: &RouteRequest) -> Option<&NetworkRoute> {
        self.routes
            .iter()
            .rev()
            .find(|route| route.rule.matches(request))
    }

    /// Selects and logs the action for `request`.
    pub fn handle(&mut self, request: RouteRequest) -> RouteAction {
        let action = self
            .match_request(&request)
            .map(|route| route.action.clone())
            .unwrap_or(RouteAction::Continue);
        self.push_log(request, action.clone());
        action
    }

    /// Returns registered routes in insertion order.
    pub fn routes(&self) -> &[NetworkRoute] {
        &self.routes
    }

    /// Returns deterministic request log entries.
    pub fn log(&self) -> &[NetworkLogEntry] {
        &self.log
    }

    pub(crate) fn push_log(&mut self, request: RouteRequest, action: RouteAction) {
        self.log
            .push(NetworkLogEntry::new(self.next_sequence, request, action));
        self.next_sequence += 1;
    }
}
