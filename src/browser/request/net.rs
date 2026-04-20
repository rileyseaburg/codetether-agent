//! Network request types for browser-resident HTTP replay and inspection.
//!
//! - [`NetworkLogRequest`] reads entries captured by the in-page `fetch`/
//!   `XMLHttpRequest` wrappers installed in `install_page_hooks`. The agent
//!   uses this to discover the exact headers (notably `Authorization:
//!   Bearer …`) the page sent.
//! - [`FetchRequest`] replays an HTTP request from inside the active tab
//!   via `page.evaluate(fetch(…))`. Because the request runs in the page's
//!   own JS context, cookies, TLS fingerprint, and Origin all match what a
//!   real user click would produce.

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct NetworkLogRequest {
    pub limit: Option<usize>,
    pub url_contains: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FetchRequest {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    /// `omit` | `same-origin` | `include` — default `include` so cookies
    /// and HttpOnly session tokens travel with the request.
    pub credentials: Option<String>,
}

/// Replay an HTTP request through the page's own axios instance.
///
/// Unlike [`FetchRequest`], this reuses every interceptor the app has
/// installed (auth header injection, CSRF tokens, baseURL rewrites,
/// request IDs), which is often the only way to reproduce a request that
/// `fetch` rejects with "Failed to fetch" due to CORS / preflight /
/// service-worker routing differences.
///
/// `axios_path` lets the caller override the discovery lookup (e.g.
/// `"window.__APP__.api"` or `"window.axios"`); when `None`, the handler
/// auto-discovers the first object on `window` that looks like an axios
/// instance (has `.defaults.baseURL` or `.defaults.headers.common`).
#[derive(Debug, Clone, Serialize)]
pub struct AxiosRequest {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
    pub axios_path: Option<String>,
}

/// Diagnostic snapshot of the page's HTTP plumbing.
///
/// Returns a JSON blob describing service workers, discovered axios
/// instances (with their `baseURL` and a sample of common headers),
/// recently-seen request initiators, and document CSP. Useful when
/// `fetch` replay fails with a transport-layer error and the agent
/// needs to decide whether to fall back to axios, hook the app's save
/// function, or call the worker directly.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DiagnoseRequest {}

/// Replay an HTTP request through a raw `XMLHttpRequest` inside the page.
///
/// Use this when [`FetchRequest`] returns "Failed to fetch" but
/// `network_log` shows the app's own successful request had `kind: xhr`.
/// The XHR transport differs from `fetch` in several ways that matter for
/// WAF / CORS / service-worker routing:
///
/// - XHR does not send `Sec-Fetch-Mode: cors` / `Sec-Fetch-Dest: empty`
///   headers the same way, which some edge rules use to block tool replays.
/// - XHR inherits the document's full cookie jar and Origin by default;
///   there is no `credentials: 'omit'` equivalent.
/// - Service workers often pass XHR through untouched while intercepting
///   `fetch`, so a SW-rewriting auth header won't affect this path.
/// - Simple XHRs (GET/POST with allowlisted headers) skip CORS preflight
///   on the same rules as the original page script.
///
/// Request body is sent verbatim; set `Content-Type` explicitly in
/// `headers` when sending JSON.
#[derive(Debug, Clone, Serialize)]
pub struct XhrRequest {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    /// When true (default), sets `xhr.withCredentials = true` so cookies
    /// and `Authorization` travel cross-origin. Set false to mimic a
    /// public-asset request.
    pub with_credentials: Option<bool>,
}

/// Replay a request captured in `window.__codetether_net_log` with
/// optional edits.
///
/// Finds the most recent entry whose URL contains [`url_contains`]
/// (and, optionally, matches [`method_filter`]), inherits its
/// captured method, URL, and request headers (including
/// `Authorization`), and re-fires via raw XHR. The captured request
/// body is used as-is unless one of:
///
/// - [`body_override`] — replaces the body with an arbitrary string
/// - [`body_patch`] — deep-merged into the captured body when it
///   parses as JSON (other keys in the captured body are preserved)
///
/// [`url_override`] / [`method_override`] replace the captured URL /
/// method if set. Headers supplied via [`extra_headers`] are overlaid
/// on top of the captured headers.
///
/// This exists so the agent can perform the common "capture one real
/// save, then re-save with different fields" workflow without
/// reconstructing the request from scratch.
///
/// [`url_contains`]: Self::url_contains
/// [`method_filter`]: Self::method_filter
/// [`url_override`]: Self::url_override
/// [`method_override`]: Self::method_override
/// [`body_override`]: Self::body_override
/// [`body_patch`]: Self::body_patch
/// [`extra_headers`]: Self::extra_headers
#[derive(Debug, Clone, Serialize)]
pub struct ReplayRequest {
    pub url_contains: String,
    pub method_filter: Option<String>,
    pub url_override: Option<String>,
    pub method_override: Option<String>,
    pub body_patch: Option<serde_json::Value>,
    pub body_override: Option<String>,
    pub extra_headers: Option<HashMap<String, String>>,
    pub with_credentials: Option<bool>,
}

