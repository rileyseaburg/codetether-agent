//! Serializable redirect steps and final cookie-jar snapshot.
use crate::browser::offline::cookie_parse::CookieRecord;
#[derive(Debug, serde::Serialize)]
/// Redirect history and final cookie state of an HTTP trace.
///
/// # Examples
/// ```
/// use codetether_browser::browser::offline::auth_trace::AuthTrace;
/// let trace = AuthTrace { final_url: "https://example.test/".into(), redirect_count: 0,
///     truncated: false, steps: vec![], cookies_after: vec![] };
/// assert!(!trace.truncated);
/// ```
pub struct AuthTrace {
    pub final_url: String,
    pub redirect_count: usize,
    pub truncated: bool,
    pub steps: Vec<Step>,
    pub cookies_after: Vec<CookieRecord>,
}
#[derive(Debug, serde::Serialize)]
/// One HTTP response and the cookie metadata observed at that hop.
///
/// # Examples
/// ```
/// use codetether_browser::browser::offline::auth_trace::Step;
/// let step = Step { method: "GET".into(), url: "https://example.test/".into(),
///     status: 200, location: None, cookie_header_sent: None, set_cookies: vec![] };
/// assert_eq!(step.status, 200);
/// ```
pub struct Step {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_header_sent: Option<String>,
    pub set_cookies: Vec<CookieRecord>,
}
