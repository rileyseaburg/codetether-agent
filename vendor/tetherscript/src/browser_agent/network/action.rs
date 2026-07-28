//! Route actions selected by the network route table.

/// Static response data returned by a fulfilled route.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RouteFulfillment;
///
/// let response = RouteFulfillment::text(201, "created");
/// assert_eq!(response.status, 201);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteFulfillment {
    /// HTTP status code to expose to the caller.
    pub status: u16,
    /// Deterministic response headers.
    pub headers: Vec<(String, String)>,
    /// Response body text.
    pub body: String,
}

impl RouteFulfillment {
    /// Builds a text response without headers.
    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: body.into(),
        }
    }
}

/// Decision returned for a matching network request.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RouteAction;
///
/// assert_eq!(RouteAction::abort("offline"), RouteAction::Abort("offline".into()));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteAction {
    /// Let the request continue without interception.
    Continue,
    /// Fail the request with a deterministic reason.
    Abort(String),
    /// Complete the request with a synthetic response.
    Fulfill(RouteFulfillment),
}

impl RouteAction {
    /// Builds an abort action.
    pub fn abort(reason: impl Into<String>) -> Self {
        Self::Abort(reason.into())
    }

    /// Builds a text fulfillment action.
    pub fn fulfill(status: u16, body: impl Into<String>) -> Self {
        Self::Fulfill(RouteFulfillment::text(status, body))
    }
}
