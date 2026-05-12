//! JS template for the diagnose action.

/// In-page script that reports service workers, axios instances,
/// document CSP, and network-log summary.
pub(super) const SCRIPT: &str = r#"(async () => {
  const out = {};
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
  try {
    const axs = [], seen = new Set();
    const inspect = (obj, path) => {
      if (!obj || typeof obj !== 'object' || seen.has(obj)) return; seen.add(obj);
      if (typeof obj.request === 'function' && obj.defaults && typeof obj.defaults === 'object')
        axs.push({ path, baseURL: obj.defaults.baseURL || null, common_headers: (obj.defaults.headers && obj.defaults.headers.common) || {}, withCredentials: !!obj.defaults.withCredentials,
          interceptor_counts: { request: (obj.interceptors&&obj.interceptors.request&&obj.interceptors.request.handlers&&obj.interceptors.request.handlers.length)||0,
            response: (obj.interceptors&&obj.interceptors.response&&obj.interceptors.response.handlers&&obj.interceptors.response.handlers.length)||0 } });
    };
    if (window.axios) inspect(window.axios, 'window.axios');
    for (const k of Object.keys(window)) { try { inspect(window[k], 'window.'+k); } catch (_) {} }
    out.axios_instances = axs;
  } catch (e) { out.axios_error = String(e); }
  try { out.document = { url: location.href, origin: location.origin, base: (document.querySelector('base')||{}).href||null,
    csp_meta: Array.from(document.querySelectorAll('meta[http-equiv="Content-Security-Policy"]')).map((m)=>m.content) };
  } catch (e) { out.document_error = String(e); }
  try { const log = (window.__codetether_net_log||[]); const by_method={}, by_kind={};
    for (const e of log) { by_method[e.method]=(by_method[e.method]||0)+1; by_kind[e.kind]=(by_kind[e.kind]||0)+1; }
    out.network_log = { total: log.length, by_method, by_kind, last: log.slice(-3) };
  } catch (e) { out.network_log_error = String(e); }
  return out;
})()"#;
