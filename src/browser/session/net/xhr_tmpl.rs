//! JS template for XHR replay with fallback hints + header inheritance.

use crate::browser::{BrowserError, request::XhrRequest};

/// Build the in-page XHR replay script from the request fields.
pub(super) fn build_script(request: XhrRequest) -> Result<String, BrowserError> {
    let headers = serde_json::to_string(&request.headers.unwrap_or_default())?;
    let url = serde_json::to_string(&request.url)?;
    let method = serde_json::to_string(&request.method.to_uppercase())?;
    let body = serde_json::to_string(&request.body)?;
    let wc = if request.with_credentials.unwrap_or(true) {
        "true"
    } else {
        "false"
    };
    Ok(TEMPLATE
        .replace("__FALLBACK_JS__", super::fallback_js::PREAMBLE)
        .replace("__METHOD__", &method)
        .replace("__URL__", &url)
        .replace("__WC__", wc)
        .replace("__HEADERS__", &headers)
        .replace("__BODY__", &body))
}

const TEMPLATE: &str = r#"(() => new Promise((resolve) => {
  __FALLBACK_JS__
  const targetUrl = __URL__, wantMethod = __METHOD__;
  const sentHeaders = __HEADERS__, presentLower = {};
  for (const k of Object.keys(sentHeaders)) presentLower[String(k).toLowerCase()] = true;
  const donor = __ct_fb_findDonor(targetUrl, wantMethod);
  const inherited = __ct_fb_inherit(donor, presentLower);
  const inheritedNames = inherited.map((h) => h.name);
  const fail = (errMsg, status, statusText) => {
    const fb = __ct_fb_suggest(errMsg, donor, inheritedNames.slice(), 'xhr');
    resolve({ ok: false, status, status_text: statusText, url: targetUrl, headers: {}, body: null,
      error: errMsg, hint: fb.hint, suggested_actions: fb.suggested_actions,
      missing_headers: inheritedNames, matched_network_log_entry: donor || null, inherited_headers: inheritedNames });
  };
  try {
    const xhr = new XMLHttpRequest();
    xhr.open(wantMethod, targetUrl, true); xhr.withCredentials = __WC__;
    for (const h of inherited) { try { xhr.setRequestHeader(h.k, h.v); } catch (_) {} }
    for (const [k, v] of Object.entries(sentHeaders)) { try { xhr.setRequestHeader(k, v); } catch (_) {} }
    xhr.onload = () => {
      const hdrs = {};
      try { (xhr.getAllResponseHeaders() || '').trim().split(/\r?\n/).forEach((l) => {
        const i = l.indexOf(':'); if (i > 0) hdrs[l.slice(0,i).trim().toLowerCase()] = l.slice(i+1).trim(); }); } catch (_) {}
      resolve({ ok: xhr.status >= 200 && xhr.status < 300, status: xhr.status, status_text: xhr.statusText,
        url: xhr.responseURL, headers: hdrs, body: xhr.responseText, inherited_headers: inheritedNames });
    };
    xhr.onerror = () => fail('xhr.onerror (likely CORS / WAF / offline)', xhr.status || 0, 'network error');
    xhr.ontimeout = () => fail('xhr timeout', 0, 'timeout');
    const body = __BODY__; xhr.send(body === null || body === undefined ? null : body);
  } catch (err) { fail(String(err&&err.stack||err), 0, String((err&&err.message)||err)); }
}))()"#;
