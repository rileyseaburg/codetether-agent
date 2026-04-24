//! Shared in-page JS helpers for replay fallback hints and
//! auto-inheritance of headers from the captured `__codetether_net_log`.
//!
//! Consumed by [`super::fetch_tmpl`] and [`super::xhr_tmpl`] via a
//! `__FALLBACK_JS__` placeholder. Keeping this in one place ensures
//! `fetch`/`xhr` stay in sync on:
//!
//! - donor-entry selection (same URL+method, prefer 2xx)
//! - header denylist (host, cookie, sec-*, etc. — browser-managed)
//! - fallback suggestion order (xhr vs axios vs diagnose)
//!
//! Header *values* never leave the page — only the inherited / missing
//! header *names* are returned so auth tokens do not leak into logs.

/// JS that defines `__ct_fb_findDonor`, `__ct_fb_inherit`, and
/// `__ct_fb_suggest` helpers. Splice into a template before the
/// replay attempt.
pub(super) const PREAMBLE: &str = r#"
const __ct_fb_SKIP = new Set(['host','content-length','connection','accept-encoding','user-agent','referer','origin','cookie','content-type']);
const __ct_fb_findDonor = (targetUrl, wantMethod) => {
  const log = (window.__codetether_net_log || []); let any = null;
  for (let i = log.length - 1; i >= 0; i--) {
    const e = log[i]; if (!e || !e.url || e.method !== wantMethod) continue;
    if (e.url !== targetUrl && e.url.indexOf(targetUrl) === -1) continue;
    any = any || e;
    if (typeof e.status === 'number' && e.status >= 200 && e.status < 400) return e;
  } return any;
};
const __ct_fb_inherit = (donor, presentLower) => {
  const out = []; if (!donor || !donor.request_headers) return out;
  for (const [k, v] of Object.entries(donor.request_headers)) {
    const kk = String(k).toLowerCase();
    if (__ct_fb_SKIP.has(kk) || kk.startsWith('sec-') || presentLower[kk]) continue;
    out.push({ k, v, name: kk });
  } return out;
};
const __ct_fb_hasAxios = () => { try { if (window.axios) return true;
  return Object.keys(window).some((k) => { try { const v = window[k];
    return v && typeof v === 'object' && typeof v.request === 'function' && v.defaults && typeof v.defaults === 'object'; } catch (_) { return false; } });
} catch (_) { return false; } };
const __ct_fb_suggest = (errMsg, donor, missing, kind) => {
  const hasAxios = __ct_fb_hasAxios();
  const miss = missing.length ? ' Missing headers vs successful request: ' + missing.join(', ') + '.' : '';
  if (kind === 'fetch' && donor && donor.kind === 'xhr')
    return { hint: 'fetch failed status 0 ('+errMsg+"). App's successful request used XHR — retry via browserctl xhr."+miss, suggested_actions: hasAxios ? ['browserctl xhr','browserctl axios'] : ['browserctl xhr'] };
  if (hasAxios) return { hint: kind+' failed status 0 ('+errMsg+'). axios is on window — retry via browserctl axios so interceptors/auth/baseURL apply.'+miss, suggested_actions: ['browserctl axios','browserctl diagnose'] };
  return { hint: kind+' failed status 0 ('+errMsg+'). Likely CORS, service-worker, or WAF. Run browserctl diagnose.'+miss, suggested_actions: kind==='fetch' ? ['browserctl xhr','browserctl diagnose'] : ['browserctl diagnose'] };
};
"#;
