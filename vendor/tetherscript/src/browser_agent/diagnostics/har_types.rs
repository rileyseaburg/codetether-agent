//! HAR-style production network report types.

/// HAR-style request entry captured from native page networking.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{
///     BrowserHarEntry, BrowserHarRequest, BrowserHarResponse, BrowserHarTimings,
/// };
///
/// let entry = BrowserHarEntry {
///     sequence: 0,
///     started_ms: 1,
///     request: BrowserHarRequest {
///         method: "GET".into(),
///         url: "/api".into(),
///         headers: Vec::new(),
///         post_data: None,
///     },
///     response: BrowserHarResponse {
///         status: 200,
///         status_text: "OK".into(),
///         headers: Vec::new(),
///         content_text: None,
///         route_result: None,
///     },
///     timings: BrowserHarTimings::default(),
/// };
/// assert_eq!(entry.response.status, 200);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserHarEntry {
    /// Monotonic entry index.
    pub sequence: u64,
    /// Deterministic wall-clock timestamp from the native session.
    pub started_ms: u128,
    /// Request metadata.
    pub request: BrowserHarRequest,
    /// Response metadata.
    pub response: BrowserHarResponse,
    /// Deterministic timing buckets.
    pub timings: BrowserHarTimings,
}

/// HAR-style request metadata.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserHarRequest;
///
/// let request = BrowserHarRequest {
///     method: "POST".into(),
///     url: "/api".into(),
///     headers: vec![("content-type".into(), "text/plain".into())],
///     post_data: Some("payload".into()),
/// };
/// assert_eq!(request.method, "POST");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserHarRequest {
    /// HTTP method.
    pub method: String,
    /// Request URL.
    pub url: String,
    /// Request headers captured by route handling.
    pub headers: Vec<(String, String)>,
    /// Optional request body preview.
    pub post_data: Option<String>,
}

/// HAR-style response metadata.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserHarResponse;
///
/// let response = BrowserHarResponse {
///     status: 404,
///     status_text: "Not Found".into(),
///     headers: Vec::new(),
///     content_text: Some("not found".into()),
///     route_result: None,
/// };
/// assert_eq!(response.status_text, "Not Found");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserHarResponse {
    /// HTTP status, or `0` for aborted/blocked requests.
    pub status: u16,
    /// Status text or route failure reason.
    pub status_text: String,
    /// Response headers captured from fulfilled routes.
    pub headers: Vec<(String, String)>,
    /// Optional response body preview.
    pub content_text: Option<String>,
    /// Route decision such as `fulfill`, `abort`, `blocked`, or `continue`.
    pub route_result: Option<String>,
}

/// Deterministic timing buckets for a native HAR-style entry.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserHarTimings;
///
/// let timings = BrowserHarTimings::default();
/// assert_eq!(timings.total_ms, 0);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BrowserHarTimings {
    /// Total deterministic request time.
    pub total_ms: u128,
    /// Send time. Native deterministic requests are currently synchronous.
    pub send_ms: u128,
    /// Wait time between request and response.
    pub wait_ms: u128,
    /// Receive time for the response body.
    pub receive_ms: u128,
}
