//! Network-log inspection and in-page HTTP replay.
//!
//! Both handlers run JS inside the active tab (`access::current_page`) so
//! the agent's requests inherit the real browser's cookies, Origin, TLS
//! fingerprint, and service-worker routing. This is what lets the agent
//! skip driving React forms and replay the exact backend call.

use super::{BrowserSession, access};
use crate::browser::{
    BrowserError, BrowserOutput,
    request::{AxiosRequest, DiagnoseRequest, FetchRequest, NetworkLogRequest},
};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

pub(super) async fn network_log(
    session: &BrowserSession,
    request: NetworkLogRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    // Best-effort: install the hook on the current page in case the tab
    // was loaded before the session attached (Connect mode) or the hook
    // script failed to inject at document-start. The IIFE is idempotent.
    let _ = super::lifecycle::install_page_hooks(&page).await;
    let filter = serde_json::json!({
        "limit": request.limit,
        "url_contains": request.url_contains,
        "method": request.method.map(|m| m.to_uppercase()),
    });
    let script = format!(
        r#"(() => {{
  const f = {filter};
  const log = (window.__codetether_net_log || []).slice();
  const limit = typeof f.limit === 'number' && f.limit > 0 ? f.limit : log.length;
  const method = f.method || null;
  const needle = f.url_contains || null;
  const out = [];
  for (let i = log.length - 1; i >= 0 && out.length < limit; i--) {{
    const e = log[i];
    if (method && e.method !== method) continue;
    if (needle && (!e.url || e.url.indexOf(needle) === -1)) continue;
    out.push(e);
  }}
  return out.reverse();
}})()"#,
        filter = filter
    );
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone().unwrap_or(serde_json::json!([]));
    Ok(BrowserOutput::Json(value))
}

pub(super) async fn fetch(
    session: &BrowserSession,
    request: FetchRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let credentials = request.credentials.unwrap_or_else(|| "include".to_string());
    let init = serde_json::json!({
        "method": request.method.to_uppercase(),
        "headers": request.headers.unwrap_or_default(),
        "body": request.body,
        "credentials": credentials,
    });
    let url = serde_json::to_string(&request.url)?;
    let init_str = init.to_string();
    let script = format!(
        r#"(async () => {{
  const init = {init_str};
  if (init.body === null) delete init.body;
  try {{
    const r = await fetch({url}, init);
    const headers = {{}};
    try {{ r.headers.forEach((v, k) => {{ headers[String(k).toLowerCase()] = String(v); }}); }} catch (_) {{}}
    let body = null;
    try {{ body = await r.text(); }} catch (_) {{}}
    return {{ ok: r.ok, status: r.status, status_text: r.statusText, url: r.url, headers, body }};
  }} catch (err) {{
    return {{ ok: false, status: 0, status_text: String((err && err.message) || err), url: {url}, headers: {{}}, body: null, error: String(err && err.stack || err) }};
  }}
}})()"#,
        url = url,
        init_str = init_str,
    );
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result
        .object()
        .value
        .clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}

/// Replay a request through the page's own axios instance so interceptors
/// (auth headers, CSRF, baseURL, request IDs) all apply. Falls back to a
/// descriptive error if no axios can be located on `window`.
pub(super) async fn axios(
    session: &BrowserSession,
    request: AxiosRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let body_json = request.body.unwrap_or(serde_json::Value::Null);
    let headers_json = serde_json::to_value(request.headers.unwrap_or_default())
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let url_json = serde_json::to_string(&request.url)?;
    let method = request.method.to_lowercase();
    // Allow methods that take no body (get/delete/head) vs body methods
    // (post/put/patch). Axios signatures differ — keep the shape close
    // to the library's canonical API rather than rewriting to .request().
    let body_methods = matches!(method.as_str(), "post" | "put" | "patch");
    let axios_path = request
        .axios_path
        .unwrap_or_else(|| "__autodetect__".to_string());
    let axios_path_json = serde_json::to_string(&axios_path)?;
    let body_str = serde_json::to_string(&body_json)?;
    let headers_str = serde_json::to_string(&headers_json)?;
    let script = format!(
        r#"(async () => {{
  const needle = {axios_path_json};
  const findAxios = () => {{
    if (needle !== '__autodetect__') {{
      try {{
        const parts = needle.replace(/^window\./, '').split('.');
        let obj = window;
        for (const p of parts) obj = obj && obj[p];
        if (obj && typeof obj.request === 'function') return {{ instance: obj, path: needle }};
      }} catch (_) {{}}
      return null;
    }}
    // Auto-discovery: walk window keys looking for an object that
    // quacks like an axios instance. Prefer ones with a baseURL so
    // relative URLs resolve.
    const seen = new Set();
    const candidates = [];
    const pushIfAxios = (obj, path) => {{
      if (!obj || typeof obj !== 'object' || seen.has(obj)) return;
      seen.add(obj);
      const isAxios = typeof obj.request === 'function'
        && obj.defaults && typeof obj.defaults === 'object';
      if (isAxios) candidates.push({{ instance: obj, path, hasBaseURL: !!obj.defaults.baseURL }});
    }};
    try {{
      if (window.axios) pushIfAxios(window.axios, 'window.axios');
      for (const k of Object.keys(window)) {{
        try {{ pushIfAxios(window[k], 'window.' + k); }} catch (_) {{}}
      }}
    }} catch (_) {{}}
    // Favor instances with a baseURL over the bare library.
    candidates.sort((a, b) => Number(b.hasBaseURL) - Number(a.hasBaseURL));
    return candidates[0] || null;
  }};
  const found = findAxios();
  if (!found) {{
    return {{ ok: false, status: 0, error: 'no axios instance on window', axios_path: null }};
  }}
  const ax = found.instance;
  const url = {url};
  const body = {body_str};
  const headers = {headers_str};
  const config = {{ headers }};
  try {{
    let resp;
    const method = {method_json};
    if (method === 'get') resp = await ax.get(url, config);
    else if (method === 'delete') resp = await ax.delete(url, config);
    else if (method === 'head') resp = await ax.head(url, config);
    else if ({body_methods}) resp = await ax[method](url, body, config);
    else resp = await ax.request({{ method, url, data: body, headers }});
    return {{
      ok: true,
      status: resp.status,
      status_text: resp.statusText,
      headers: resp.headers || {{}},
      data: resp.data,
      axios_path: found.path,
    }};
  }} catch (err) {{
    const resp = err && err.response;
    return {{
      ok: false,
      status: resp ? resp.status : 0,
      status_text: resp ? resp.statusText : String((err && err.message) || err),
      headers: (resp && resp.headers) || {{}},
      data: resp && resp.data,
      error: String((err && err.stack) || (err && err.message) || err),
      axios_path: found.path,
    }};
  }}
}})()"#,
        axios_path_json = axios_path_json,
        url = url_json,
        body_str = body_str,
        headers_str = headers_str,
        method_json = serde_json::to_string(&method)?,
        body_methods = body_methods,
    );
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result
        .object()
        .value
        .clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}

/// Dump the page's HTTP plumbing: service workers, discovered axios
/// instances (with baseURL + common headers), document CSP, and a
/// count of recent network-log entries per initiator.
pub(super) async fn diagnose(
    session: &BrowserSession,
    _request: DiagnoseRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    // Best-effort hook install so the network log isn't empty on fresh tabs.
    let _ = super::lifecycle::install_page_hooks(&page).await;
    const SCRIPT: &str = r#"(async () => {
  const out = {};
  // Service workers — these intercept fetch and are a common cause of
  // 'Failed to fetch' replays.
  try {
    if (navigator.serviceWorker && navigator.serviceWorker.getRegistrations) {
      const regs = await navigator.serviceWorker.getRegistrations();
      out.service_workers = regs.map((r) => ({
        scope: r.scope,
        active: r.active && { state: r.active.state, script: r.active.scriptURL },
        waiting: r.waiting && { state: r.waiting.state, script: r.waiting.scriptURL },
        installing: r.installing && { state: r.installing.state, script: r.installing.scriptURL },
      }));
    } else { out.service_workers = []; }
  } catch (e) { out.service_workers_error = String(e); }
  // Axios instances on window.
  try {
    const axs = [];
    const seen = new Set();
    const inspect = (obj, path) => {
      if (!obj || typeof obj !== 'object' || seen.has(obj)) return;
      seen.add(obj);
      if (typeof obj.request === 'function' && obj.defaults && typeof obj.defaults === 'object') {
        axs.push({
          path,
          baseURL: obj.defaults.baseURL || null,
          common_headers: (obj.defaults.headers && obj.defaults.headers.common) || {},
          withCredentials: !!obj.defaults.withCredentials,
          interceptor_counts: {
            request: (obj.interceptors && obj.interceptors.request && obj.interceptors.request.handlers && obj.interceptors.request.handlers.length) || 0,
            response: (obj.interceptors && obj.interceptors.response && obj.interceptors.response.handlers && obj.interceptors.response.handlers.length) || 0,
          },
        });
      }
    };
    if (window.axios) inspect(window.axios, 'window.axios');
    for (const k of Object.keys(window)) {
      try { inspect(window[k], 'window.' + k); } catch (_) {}
    }
    out.axios_instances = axs;
  } catch (e) { out.axios_error = String(e); }
  // Document CSP + origin + base.
  try {
    out.document = {
      url: location.href,
      origin: location.origin,
      base: (document.querySelector('base') || {}).href || null,
      csp_meta: Array.from(document.querySelectorAll('meta[http-equiv="Content-Security-Policy"]')).map((m) => m.content),
    };
  } catch (e) { out.document_error = String(e); }
  // Network log summary.
  try {
    const log = (window.__codetether_net_log || []);
    const by_method = {};
    const by_kind = {};
    for (const e of log) {
      by_method[e.method] = (by_method[e.method] || 0) + 1;
      by_kind[e.kind] = (by_kind[e.kind] || 0) + 1;
    }
    out.network_log = { total: log.length, by_method, by_kind, last: log.slice(-3) };
  } catch (e) { out.network_log_error = String(e); }
  return out;
})()"#;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(SCRIPT))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result
        .object()
        .value
        .clone()
        .unwrap_or(serde_json::json!({}));
    Ok(BrowserOutput::Json(value))
}
