//! JS template for `fetch()` replay with fallback hints + header inheritance.

use crate::browser::{BrowserError, request::FetchRequest};

/// Build the in-page `fetch` replay script from the request fields.
pub(super) fn build_script(request: FetchRequest) -> Result<String, BrowserError> {
    let credentials = request.credentials.unwrap_or_else(|| "include".to_string());
    let init = serde_json::json!({
        "method": request.method.to_uppercase(),
        "headers": request.headers.unwrap_or_default(),
        "body": request.body, "credentials": credentials,
    });
    let url = serde_json::to_string(&request.url)?;
    Ok(TEMPLATE
        .replace("__FALLBACK_JS__", super::fallback_js::PREAMBLE)
        .replace("__URL__", &url)
        .replace("__INIT__", &init.to_string()))
}

const TEMPLATE: &str = r#"(async () => {
  __FALLBACK_JS__
  const init = __INIT__; if (init.body === null) delete init.body;
  const targetUrl = __URL__;
  const wantMethod = (init.method || 'GET').toUpperCase();
  const presentLower = {}; init.headers = init.headers || {};
  for (const k of Object.keys(init.headers)) presentLower[String(k).toLowerCase()] = true;
  const donor = __ct_fb_findDonor(targetUrl, wantMethod);
  const inherited = __ct_fb_inherit(donor, presentLower);
  const inheritedNames = [];
  for (const h of inherited) { init.headers[h.k] = h.v; inheritedNames.push(h.name); }
  try {
    const r = await fetch(targetUrl, init);
    const hdrs = {}; try { r.headers.forEach((v, k) => { hdrs[String(k).toLowerCase()] = String(v); }); } catch (_) {}
    let body = null; try { body = await r.text(); } catch (_) {}
    return { ok: r.ok, status: r.status, status_text: r.statusText, url: r.url, headers: hdrs, body, inherited_headers: inheritedNames };
  } catch (err) {
    const errMsg = String((err && err.message) || err);
    const missing = inheritedNames.slice();
    const fb = __ct_fb_suggest(errMsg, donor, missing, 'fetch');
    return { ok: false, status: 0, status_text: errMsg, url: targetUrl, headers: {}, body: null,
      error: String(err && err.stack || err), hint: fb.hint, suggested_actions: fb.suggested_actions,
      missing_headers: missing, matched_network_log_entry: donor || null, inherited_headers: inheritedNames };
  }
})()"#;
