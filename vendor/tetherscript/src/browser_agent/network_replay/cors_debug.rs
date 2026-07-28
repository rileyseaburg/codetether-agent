//! Inline CORS analysis with explicit block reasons.

use super::capture::{header, CapturedExchange, Headers};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorsAnalysis {
    pub blocked: bool, pub reason: CorsBlockReason,
    pub preflight: Option<PreflightAnalysis>,
    pub allowed_origins: Vec<String>, pub actual_origin: String,
    pub credentials_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CorsBlockReason {
    None, MissingAllowOrigin, AllowOriginMismatch,
    MissingAllowCredentials, MethodNotAllowed, HeaderNotAllowed, PreflightNetworkError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreflightAnalysis {
    pub request_headers: Headers, pub response_headers: Headers,
    pub allowed_methods: Vec<String>, pub allowed_headers: Vec<String>,
    pub max_age: Option<u64>,
}

/// Analyze CORS behavior for an actual request, optionally with its preflight.
pub fn analyze_cors(actual: &CapturedExchange, preflight: Option<&CapturedExchange>) -> CorsAnalysis {
    let origin = header(&actual.request.headers, "origin").unwrap_or("").to_string();
    let creds = header(&actual.request.headers, "cookie").is_some()
        || header(&actual.request.headers, "authorization").is_some();
    let mut pf = None;
    if let Some(p) = preflight {
        if p.response.is_none() { return out(true, CorsBlockReason::PreflightNetworkError, None, vec![], origin, creds); }
        let r = p.response.as_ref().unwrap();
        let methods = list(header(&r.headers, "access-control-allow-methods"));
        let hdrs = list(header(&r.headers, "access-control-allow-headers"));
        pf = Some(PreflightAnalysis { request_headers: p.request.headers.clone(), response_headers: r.headers.clone(),
            allowed_methods: methods.clone(), allowed_headers: hdrs.clone(),
            max_age: header(&r.headers, "access-control-max-age").and_then(|s| s.parse().ok()) });
        if !methods.is_empty() && !methods.iter().any(|m| m.eq_ignore_ascii_case(&actual.request.method)) {
            return out(true, CorsBlockReason::MethodNotAllowed, pf, vec![], origin, creds);
        }
    }
    let resp_headers = actual.response.as_ref().map(|r| &r.headers);
    let allow = resp_headers.and_then(|h| header(h, "access-control-allow-origin"));
    let origins = allow.map(|s| vec![s.to_string()]).unwrap_or_default();
    if allow.is_none() { return out(true, CorsBlockReason::MissingAllowOrigin, pf, origins, origin, creds); }
    if allow != Some("*") && allow != Some(origin.as_str()) { return out(true, CorsBlockReason::AllowOriginMismatch, pf, origins, origin, creds); }
    if creds && resp_headers.map_or(false, |h| header(h, "access-control-allow-credentials") != Some("true")) {
        return out(true, CorsBlockReason::MissingAllowCredentials, pf, origins, origin, creds);
    }
    out(false, CorsBlockReason::None, pf, origins, origin, creds)
}

fn out(blocked: bool, reason: CorsBlockReason, preflight: Option<PreflightAnalysis>,
    allowed_origins: Vec<String>, actual_origin: String, credentials_included: bool) -> CorsAnalysis {
    CorsAnalysis { blocked, reason, preflight, allowed_origins, actual_origin, credentials_included }
}
fn list(v: Option<&str>) -> Vec<String> { v.unwrap_or("").split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect() }
