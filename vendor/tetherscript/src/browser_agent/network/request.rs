//! Request and log records used by route tables.

use super::RouteAction;
use crate::browser_agent::exports::RequestSecurityMetadata;

/// Minimal request metadata used for route matching.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RouteRequest;
///
/// let request = RouteRequest::new("post", "https://example.test/api");
/// assert_eq!(request.method, "POST");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteRequest {
    /// Uppercase HTTP method.
    pub method: String,
    /// Absolute or page-relative request URL.
    pub url: String,
    /// Request headers in runtime-normalized form.
    pub headers: Vec<(String, String)>,
    /// Optional text request body.
    pub body: Option<String>,
    /// Origin/referrer metadata captured at the page boundary.
    pub security: Option<RequestSecurityMetadata>,
}

impl RouteRequest {
    /// Creates request metadata for route matching.
    pub fn new(method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            method: method.into().to_ascii_uppercase(),
            url: url.into(),
            headers: Vec::new(),
            body: None,
            security: None,
        }
    }

    /// Attach request headers to the route metadata.
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }

    /// Attach a text body to the route metadata.
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub(crate) fn with_optional_body(mut self, body: Option<String>) -> Self {
        self.body = body;
        self
    }

    /// Attach page security metadata to the request.
    pub fn with_security(mut self, security: RequestSecurityMetadata) -> Self {
        self.security = Some(security);
        self
    }
}

/// Deterministic HAR-lite record emitted when a table handles a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkLogEntry {
    /// Monotonic request sequence number.
    pub sequence: u64,
    /// Matched request method.
    pub method: String,
    /// Matched request URL.
    pub url: String,
    /// Request headers observed by routing.
    pub headers: Vec<(String, String)>,
    /// Optional text request body observed by routing.
    pub body: Option<String>,
    /// Action selected for the request.
    pub action: RouteAction,
    /// Security metadata observed for a page-initiated request.
    pub security: Option<RequestSecurityMetadata>,
}
