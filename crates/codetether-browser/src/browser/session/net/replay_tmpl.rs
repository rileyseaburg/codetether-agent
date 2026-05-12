//! JS template for replay-from-captured-log.

use crate::browser::{BrowserError, request::ReplayRequest};

pub(super) fn build_script(request: ReplayRequest) -> Result<String, BrowserError> {
    let needle = serde_json::to_string(&request.url_contains)?;
    let method_filter = serde_json::to_string(&request.method_filter)?;
    let url_override = serde_json::to_string(&request.url_override)?;
    let method_override = serde_json::to_string(&request.method_override)?;
    let body_patch = serde_json::to_string(&request.body_patch)?;
    let body_override = serde_json::to_string(&request.body_override)?;
    let extra_headers = serde_json::to_string(&request.extra_headers.unwrap_or_default())?;
    let wc = if request.with_credentials.unwrap_or(true) {
        "true"
    } else {
        "false"
    };
    Ok(TEMPLATE
        .replace("__NEEDLE__", &needle)
        .replace("__METHOD_FILTER__", &method_filter)
        .replace("__URL_OVERRIDE__", &url_override)
        .replace("__METHOD_OVERRIDE__", &method_override)
        .replace("__BODY_PATCH__", &body_patch)
        .replace("__BODY_OVERRIDE__", &body_override)
        .replace("__EXTRA_HEADERS__", &extra_headers)
        .replace("__WC__", wc))
}

pub(super) const TEMPLATE: &str = r#"(() => new Promise((resolve) => {
  const log = (window.__codetether_net_log || []);
  const needle = __NEEDLE__, mFilter = __METHOD_FILTER__;
  let entry = null;
  for (let i = log.length - 1; i >= 0; i--) {
    const e = log[i]; if (!e || !e.url || e.url.indexOf(needle) === -1) continue;
    if (mFilter && String(e.method).toUpperCase() !== String(mFilter).toUpperCase()) continue;
    entry = e; break;
  }
  if (!entry) { resolve({ ok: false, status: 0, error: 'no matching log entry', needle, method_filter: mFilter }); return; }
  const method = (__METHOD_OVERRIDE__ || entry.method || 'GET').toUpperCase();
  const url = __URL_OVERRIDE__ || entry.url;
  const baseHeaders = Object.assign({}, entry.request_headers || {}, __EXTRA_HEADERS__ || {});
  let body = __BODY_OVERRIDE__;
  if (body === null || body === undefined) {
    body = entry.request_body;
    const patch = __BODY_PATCH__;
    if (patch && typeof patch === 'object' && typeof body === 'string') {
      try {
        const base = JSON.parse(body);
        const deep = (a, b) => { if (!b || typeof b !== 'object' || Array.isArray(b)) return b;
          const o = (a && typeof a === 'object' && !Array.isArray(a)) ? Object.assign({}, a) : {};
          for (const k of Object.keys(b)) o[k] = deep(o[k], b[k]); return o; };
        body = JSON.stringify(deep(base, patch));
        if (!baseHeaders['content-type']) baseHeaders['content-type'] = 'application/json';
      } catch (_) { /* body not JSON — fall back to raw */ }
    }
  }
  try {
    const xhr = new XMLHttpRequest();
    xhr.open(method, url, true); xhr.withCredentials = __WC__;
    for (const [k, v] of Object.entries(baseHeaders)) { try { xhr.setRequestHeader(k, v); } catch (_) {} }
    xhr.onload = () => { const h = {}; try { (xhr.getAllResponseHeaders()||'').trim().split(/\r?\n/).forEach((l)=>{const i=l.indexOf(':');if(i>0)h[l.slice(0,i).trim().toLowerCase()]=l.slice(i+1).trim();}); } catch(_){}
      resolve({ ok: xhr.status>=200 && xhr.status<300, status: xhr.status, status_text: xhr.statusText, url: xhr.responseURL, headers: h, body: xhr.responseText, used_method: method, used_url: url, inherited_headers: Object.keys(baseHeaders), matched_log_id: entry.id }); };
    xhr.onerror = () => resolve({ ok:false, status: xhr.status||0, error:'xhr.onerror', used_method: method, used_url: url, matched_log_id: entry.id });
    xhr.ontimeout = () => resolve({ ok:false, status:0, error:'xhr timeout', used_method: method, used_url: url, matched_log_id: entry.id });
    xhr.send(body == null ? null : body);
  } catch (err) { resolve({ ok:false, status:0, error: String(err&&err.message||err) }); }
}))()"#;
