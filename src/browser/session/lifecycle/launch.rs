use super::{browser, handler, mode::StartMode};
use crate::browser::{
    BrowserError,
    session::{SessionMode, SessionRuntime},
};
use chromiumoxide::cdp::browser_protocol::target::CloseTargetParams;
use chromiumoxide::page::Page;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub(super) async fn runtime(mode: StartMode) -> Result<SessionRuntime, BrowserError> {
    let (browser, handler, mode) = browser::connect_or_launch(mode).await?;
    let (alive, shutdown, handler_task) = handler::spawn(handler);
    let page = match initial_page(&browser).await {
        Ok(page) => page,
        Err(error) => {
            let _ = shutdown.send(true);
            handler_task.abort();
            let _ = handler_task.await;
            return Err(error);
        }
    };
    if mode == SessionMode::Launch {
        // Strip the "HeadlessChrome" token from the user agent so we look
        // like an ordinary Chrome install. Errors are non-fatal — we'd
        // rather keep the session alive with the default UA than refuse
        // to start.
        if let Err(err) = apply_stealth_ua(&page).await {
            tracing::warn!(error = %err, "failed to apply stealth user agent");
        }
    }
    // Network-log hooks must run in *both* Launch and Connect mode, and
    // must apply to the *current* document in addition to future
    // navigations — otherwise `network_log` returns [] until the next
    // reload. The hook script is idempotent via `__codetether_netidle_installed`.
    if let Err(err) = install_page_hooks(&page).await {
        tracing::warn!(error = %err, "failed to install page hooks");
    }
    if mode == SessionMode::Launch {
        if let Err(error) = prune_startup_pages(&browser, &page).await {
            let _ = shutdown.send(true);
            handler_task.abort();
            let _ = handler_task.await;
            return Err(error);
        }
    }
    let target_id = page.target_id().clone();
    Ok(SessionRuntime {
        alive,
        browser: Arc::new(Mutex::new(browser)),
        current_page: Arc::new(Mutex::new(Some(page))),
        handler_task,
        mode,
        shutdown,
        tab_order: Arc::new(Mutex::new(vec![target_id])),
    })
}

async fn initial_page(browser: &chromiumoxide::browser::Browser) -> Result<Page, BrowserError> {
    if let Some(page) = browser.pages().await?.into_iter().next() {
        return Ok(page);
    }
    Ok(browser.new_page("about:blank").await?)
}

async fn prune_startup_pages(
    browser: &chromiumoxide::browser::Browser,
    keep: &Page,
) -> Result<(), BrowserError> {
    for _ in 0..10 {
        let mut closed = false;
        for page in browser.pages().await? {
            if page.target_id() == keep.target_id() {
                continue;
            }
            let url = page.url().await?.unwrap_or_default();
            if !matches!(
                url.as_str(),
                "" | "about:blank" | "chrome://new-tab-page" | "chrome://new-tab-page/"
            ) {
                continue;
            }
            browser
                .execute(CloseTargetParams::new(page.target_id().clone()))
                .await?;
            closed = true;
        }
        if !closed {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

/// Fetch the current user agent via CDP and, if it contains "HeadlessChrome"
/// or "Headless", replace that token with plain "Chrome". Also sets a
/// plausible Accept-Language so pages don't see a blank header.
///
/// Called on every page we create (initial page + each `tabs_new`) because
/// `Network.setUserAgentOverride` is page-scoped — there is no browser-wide
/// CDP equivalent.
pub(in crate::browser::session) async fn apply_stealth_ua(
    page: &Page,
) -> Result<(), BrowserError> {
    use chromiumoxide::cdp::browser_protocol::network::SetUserAgentOverrideParams;
    let raw: String = page
        .evaluate("navigator.userAgent")
        .await?
        .into_value::<String>()
        .unwrap_or_default();
    if raw.is_empty() {
        return Ok(());
    }
    let cleaned = raw
        .replace("HeadlessChrome", "Chrome")
        .replace("Headless", "");
    if cleaned == raw {
        return Ok(());
    }
    let params = SetUserAgentOverrideParams::builder()
        .user_agent(cleaned)
        .accept_language("en-US,en;q=0.9")
        .build()
        .map_err(BrowserError::OperationFailed)?;
    page.execute(params).await?;
    Ok(())
}

/// Install in-page observers that the agent's tools rely on to behave
/// deterministically on modern SPAs. Runs via `Page.addScriptToEvaluateOnNewDocument`
/// so it fires on **every** navigation before any site JS, guaranteeing
/// that first-paint fetches are counted by the network-idle tracker and
/// that `navigator.webdriver` returns `false` before any bot-detection
/// script samples it.
///
/// Failure is always best-effort: if the script fails to install we
/// fall back to the lazy per-wait install in `wait::wait_network_idle`.
pub(in crate::browser::session) async fn install_page_hooks(
    page: &Page,
) -> Result<(), BrowserError> {
    // Keep the script compact — it runs at document-start on every
    // navigation, so cost matters.
    const HOOK_SCRIPT: &str = r#"
        (() => {
            // --- Stealth: hide automation fingerprint -------------------
            // Every one of these is a documented bot-detection signal
            // (navigator.webdriver, plugins.length===0, languages===[],
            // missing window.chrome, permissions mismatch on notifications,
            // WebGL UNMASKED_VENDOR reporting "SwiftShader"/"Google Inc.").
            try {
                Object.defineProperty(navigator, 'webdriver', { get: () => false });
            } catch (_) { /* already defined */ }
            try {
                Object.defineProperty(navigator, 'languages', {
                    get: () => ['en-US', 'en'],
                });
            } catch (_) {}
            try {
                // Non-empty plugins array; the exact values don't matter,
                // just that length > 0 and it quacks like PluginArray.
                const fakePlugins = [
                    { name: 'PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format' },
                    { name: 'Chrome PDF Viewer', filename: 'internal-pdf-viewer', description: '' },
                    { name: 'Chromium PDF Viewer', filename: 'internal-pdf-viewer', description: '' },
                ];
                Object.defineProperty(navigator, 'plugins', {
                    get: () => fakePlugins,
                });
                Object.defineProperty(navigator, 'mimeTypes', {
                    get: () => [{ type: 'application/pdf', suffixes: 'pdf' }],
                });
            } catch (_) {}
            try {
                if (!window.chrome) {
                    window.chrome = { runtime: {}, app: { isInstalled: false } };
                }
            } catch (_) {}
            try {
                // Classic notifications-permission sniff: a headless browser
                // reports Notification.permission === 'denied' while
                // permissions.query({name:'notifications'}) returns 'prompt'.
                const origQuery = window.navigator.permissions && window.navigator.permissions.query;
                if (origQuery) {
                    window.navigator.permissions.query = (params) =>
                        params && params.name === 'notifications'
                            ? Promise.resolve({ state: Notification.permission, onchange: null })
                            : origQuery.call(window.navigator.permissions, params);
                }
            } catch (_) {}
            try {
                // WebGL vendor/renderer spoof: headless Chrome advertises
                // "Google Inc." / "SwiftShader" which is a strong bot signal.
                const getParam = WebGLRenderingContext.prototype.getParameter;
                WebGLRenderingContext.prototype.getParameter = function(p) {
                    if (p === 37445) return 'Intel Inc.';        // UNMASKED_VENDOR_WEBGL
                    if (p === 37446) return 'Intel Iris OpenGL Engine'; // UNMASKED_RENDERER_WEBGL
                    return getParam.call(this, p);
                };
            } catch (_) {}
            // --- Network-idle tracker + request log ---------------------
            // Wraps fetch/XHR so the agent can (a) wait for network-idle
            // and (b) inspect the method/url/headers/body/status of each
            // request in the page via `network_log`. This is how we learn
            // the live `Authorization: Bearer …` header without CDP
            // Network events.
            if (window.__codetether_netidle_installed) return;
            window.__codetether_netidle_installed = true;
            window.__codetether_in_flight = 0;
            window.__codetether_last_change = Date.now();
            window.__codetether_net_log = window.__codetether_net_log || [];
            window.__codetether_net_seq = window.__codetether_net_seq || 0;
            const NET_LOG_MAX = 250;
            const ctPushLog = (entry) => {
                entry.id = ++window.__codetether_net_seq;
                entry.ts = Date.now();
                window.__codetether_net_log.push(entry);
                const extra = window.__codetether_net_log.length - NET_LOG_MAX;
                if (extra > 0) window.__codetether_net_log.splice(0, extra);
                return entry;
            };
            const ctHeadersToObj = (h) => {
                const out = {};
                if (!h) return out;
                try {
                    if (typeof h.forEach === 'function') {
                        h.forEach((v, k) => { out[String(k).toLowerCase()] = String(v); });
                    } else if (Array.isArray(h)) {
                        for (const pair of h) { if (pair && pair.length >= 2) out[String(pair[0]).toLowerCase()] = String(pair[1]); }
                    } else if (typeof h === 'object') {
                        for (const k of Object.keys(h)) out[String(k).toLowerCase()] = String(h[k]);
                    }
                } catch (_) {}
                return out;
            };
            const ctClipBody = (b) => {
                if (b == null) return null;
                if (typeof b === 'string') return b.length > 1048576 ? b.slice(0, 1048576) + '…' : b;
                try { return '[' + (b.constructor && b.constructor.name) + ']'; } catch (_) { return '[body]'; }
            };
            const bump = (delta) => {
                window.__codetether_in_flight = Math.max(0, window.__codetether_in_flight + delta);
                window.__codetether_last_change = Date.now();
            };
            const origFetch = window.fetch;
            if (origFetch) {
                window.fetch = function(input, init) {
                    bump(1);
                    const started = Date.now();
                    const url = typeof input === 'string' ? input : (input && input.url) || '';
                    const method = (init && init.method) || (typeof input === 'object' && input && input.method) || 'GET';
                    const reqHeaders = ctHeadersToObj(init && init.headers);
                    const entry = ctPushLog({ kind: 'fetch', method: String(method).toUpperCase(), url, request_headers: reqHeaders, request_body: ctClipBody(init && init.body), status: null, response_headers: {}, duration_ms: null });
                    return origFetch.apply(this, arguments).then((resp) => {
                        try { entry.status = resp.status; entry.response_headers = ctHeadersToObj(resp.headers); } catch (_) {}
                        return resp;
                    }).catch((err) => {
                        try { entry.error = String((err && err.message) || err); } catch (_) {}
                        throw err;
                    }).finally(() => { entry.duration_ms = Date.now() - started; bump(-1); });
                };
            }
            const XHR = window.XMLHttpRequest;
            if (XHR && XHR.prototype && XHR.prototype.open && XHR.prototype.send) {
                const origOpen = XHR.prototype.open;
                const origSet = XHR.prototype.setRequestHeader;
                const origSend = XHR.prototype.send;
                XHR.prototype.open = function(method, url) {
                    this.__ct_req = { method: String(method || 'GET').toUpperCase(), url: String(url || ''), request_headers: {}, request_body: null };
                    return origOpen.apply(this, arguments);
                };
                XHR.prototype.setRequestHeader = function(k, v) {
                    if (this.__ct_req) this.__ct_req.request_headers[String(k).toLowerCase()] = String(v);
                    return origSet.apply(this, arguments);
                };
                XHR.prototype.send = function(body) {
                    bump(1);
                    const started = Date.now();
                    const ctx = this.__ct_req || { method: 'GET', url: '', request_headers: {} };
                    ctx.request_body = ctClipBody(body);
                    const entry = ctPushLog({ kind: 'xhr', method: ctx.method, url: ctx.url, request_headers: ctx.request_headers, request_body: ctx.request_body, status: null, response_headers: {}, duration_ms: null });
                    const xhr = this;
                    const finish = () => {
                        try {
                            entry.status = xhr.status;
                            const raw = xhr.getAllResponseHeaders() || '';
                            const h = {};
                            raw.split(/\r?\n/).forEach((line) => { const i = line.indexOf(':'); if (i > 0) h[line.slice(0, i).trim().toLowerCase()] = line.slice(i + 1).trim(); });
                            entry.response_headers = h;
                        } catch (_) {}
                        entry.duration_ms = Date.now() - started;
                        bump(-1);
                    };
                    this.addEventListener('loadend', finish, { once: true });
                    this.addEventListener('abort', () => bump(-1), { once: true });
                    this.addEventListener('error', () => bump(-1), { once: true });
                    return origSend.apply(this, arguments);
                };
            }
        })();
    "#;
    page.evaluate_on_new_document(HOOK_SCRIPT.to_string()).await?;
    // Also run immediately on the current document — without this, the
    // hook only takes effect after the next navigation, which means
    // `network_log` is empty on the tab the user is already looking at.
    // The IIFE early-returns on `__codetether_netidle_installed` so
    // re-running it on every tab switch / goto is safe.
    if let Err(err) = page.evaluate(HOOK_SCRIPT).await {
        tracing::debug!(error = %err, "immediate hook eval failed (page may still be loading)");
    }
    Ok(())
}
