//! Route identifiers and matching rules.

use super::{RouteAction, RoutePattern, RouteRequest};

/// Stable identifier returned when a route is registered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RouteId(pub u64);

/// Method and URL rule for a network route.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RoutePattern, RouteRequest, RouteRule};
///
/// let rule = RouteRule::new(RoutePattern::substring("/api")).method("POST");
/// assert!(rule.matches(&RouteRequest::new("post", "/api/items")));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteRule {
    /// Optional uppercase HTTP method constraint.
    pub method: Option<String>,
    /// URL pattern constraint.
    pub pattern: RoutePattern,
}

impl RouteRule {
    /// Creates a method-agnostic rule for `pattern`.
    pub fn new(pattern: RoutePattern) -> Self {
        Self {
            method: None,
            pattern,
        }
    }

    /// Restricts this rule to one HTTP method.
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into().to_ascii_uppercase());
        self
    }

    /// Returns whether `request` satisfies the method and URL constraints.
    pub fn matches(&self, request: &RouteRequest) -> bool {
        let method_matches = match &self.method {
            Some(method) => method == &request.method,
            None => true,
        };
        method_matches && self.pattern.matches(&request.url)
    }
}

/// Registered route entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRoute {
    /// Stable table-local route id.
    pub id: RouteId,
    /// Matching rule.
    pub rule: RouteRule,
    /// Action selected when the rule matches.
    pub action: RouteAction,
}
