use super::{BrowserSession, access, shadow};
use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::WaitRequest};
use chromiumoxide::page::Page;
use std::time::Duration;
use tokio::time::Instant;

/// Multiplexes between selector / text / url / state waits depending on which
/// fields are set on the request. The schema only allows one wait predicate
/// at a time but we evaluate them in priority order anyway so the most
/// specific wins if a caller sets several.
pub(super) async fn for_selector(
    session: &BrowserSession,
    request: WaitRequest,
) -> Result<BrowserOutput, BrowserError> {
    if request
        .frame_selector
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return Err(BrowserError::OperationFailed(
            "frame-scoped waits are not implemented yet".into(),
        ));
    }
    let timeout_ms = request.timeout_ms;
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let page = access::current_page(session).await?;
    if let Some(text) = request.text.as_deref() {
        return wait_text(
            &page,
            request.selector.as_deref(),
            text,
            true,
            deadline,
            timeout_ms,
        )
        .await;
    }
    if let Some(text) = request.text_gone.as_deref() {
        return wait_text(
            &page,
            request.selector.as_deref(),
            text,
            false,
            deadline,
            timeout_ms,
        )
        .await;
    }
    if let Some(needle) = request.url_contains.as_deref() {
        return wait_url(&page, needle, deadline, timeout_ms).await;
    }
    if let Some(selector) = request.selector.as_deref() {
        return wait_selector(&page, selector, &request.state, deadline, timeout_ms).await;
    }
    // State-only waits that don't need a selector: network idle, DOM ready.
    match request.state.as_str() {
        "idle" | "network_idle" | "networkidle" => {
            return wait_network_idle(&page, deadline, timeout_ms).await;
        }
        "ready" | "domcontentloaded" => {
            return wait_ready_state(&page, &["interactive", "complete"], deadline, timeout_ms)
                .await;
        }
        "load" | "complete" => {
            return wait_ready_state(&page, &["complete"], deadline, timeout_ms).await;
        }
        _ => {}
    }
    Err(BrowserError::OperationFailed(
        "wait requires one of: selector, text, text_gone, url_contains, or state in {idle, ready, load}".into(),
    ))
}

async fn wait_selector(
    page: &Page,
    selector: &str,
    state: &str,
    deadline: Instant,
    timeout_ms: u64,
) -> Result<BrowserOutput, BrowserError> {
    let want_present = matches!(state, "" | "attached" | "visible");
    let want_visible = state == "visible" || state == "hidden";
    let invert_visible = state == "hidden" || state == "detached";
    loop {
        let present = is_attached(page, selector).await?;
        let satisfied = match (present, want_present) {
            (true, true) => {
                if want_visible {
                    let visible = is_visible(page, selector).await.unwrap_or(false);
                    if invert_visible { !visible } else { visible }
                } else {
                    true
                }
            }
            (false, false) => true,
            (false, true) => false,
            (true, false) => {
                if want_visible {
                    let visible = is_visible(page, selector).await.unwrap_or(false);
                    !visible
                } else {
                    false
                }
            }
        };
        if satisfied {
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::WaitTimeout {
                selector: selector.to_string(),
                timeout_ms,
            });
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Shadow-aware presence probe. Returns `Ok(true)` when the selector
/// resolves anywhere in the page, including inside shadow roots and via
/// the `>>>` combinator; `Ok(false)` when nothing matches; an error only
/// when the page eval itself fails. Does not check visibility — pair
/// with [`is_visible`] when the caller cares.
async fn is_attached(page: &Page, selector: &str) -> Result<bool, BrowserError> {
    let lit = serde_json::to_string(selector)?;
    let dq = shadow::dq_call(&lit);
    let script = format!("(() => Boolean({dq}))()");
    Ok(page
        .evaluate(script)
        .await?
        .into_value::<bool>()
        .unwrap_or(false))
}

/// Probe element visibility via JS so we account for `display:none`,
/// `visibility:hidden`, zero-size boxes, and offscreen position. CDP
/// `find_element` only checks attachment, not visibility.
async fn is_visible(page: &Page, selector: &str) -> Result<bool, BrowserError> {
    let lit = serde_json::to_string(selector)?;
    let dq = shadow::dq_call(&lit);
    let script = format!(
        "(() => {{
            const el = {dq};
            if (!el) return false;
            const r = el.getBoundingClientRect();
            if (r.width === 0 || r.height === 0) return false;
            const s = getComputedStyle(el);
            return s.visibility !== 'hidden' && s.display !== 'none' && parseFloat(s.opacity) > 0;
        }})()"
    );
    Ok(page
        .evaluate(script)
        .await?
        .into_value::<bool>()
        .unwrap_or(false))
}

async fn wait_text(
    page: &Page,
    scope: Option<&str>,
    needle: &str,
    want_present: bool,
    deadline: Instant,
    timeout_ms: u64,
) -> Result<BrowserOutput, BrowserError> {
    let scope_lit = serde_json::to_string(scope.unwrap_or(""))?;
    let needle_lit = serde_json::to_string(needle)?;
    // Install a MutationObserver BEFORE the polling loop so even a toast that
    // appears-and-disappears between polls (common for "Saved!" flashes that
    // the host app renders for ~200 ms) is recorded. The observer sets a
    // persistent `__codetether_seen_text[needle] = true` flag the moment any
    // node containing `needle` is inserted or mutated. For "text present"
    // checks we OR the observer's memory with a live DOM query; for
    // "text gone" we only care about the live DOM so we skip the observer.
    //
    // The sentinel is idempotent and per-needle, so repeated waits on the
    // same page don't cross-contaminate each other.
    let dq_install = shadow::DEEP_QUERY_INSTALL;
    if want_present {
        let install = format!(
            "(() => {{
                const dq = ({dq_install});
                window.__codetether_seen_text = window.__codetether_seen_text || {{}};
                if (window.__codetether_seen_text[{needle_lit}]) return true;
                const root = {scope_lit} ? dq({scope_lit}) : document.body;
                if (!root) return false;
                // Already visible in the DOM right now?
                const current = (root.innerText || root.textContent || '');
                if (current.includes({needle_lit})) {{
                    window.__codetether_seen_text[{needle_lit}] = true;
                    return true;
                }}
                // Install an observer (only once per needle).
                const key = '__codetether_observer_' + btoa(unescape(encodeURIComponent({needle_lit}))).replace(/=/g,'');
                if (!window[key]) {{
                    const obs = new MutationObserver(() => {{
                        try {{
                            const r = {scope_lit} ? dq({scope_lit}) : document.body;
                            if (!r) return;
                            const t = (r.innerText || r.textContent || '');
                            if (t.includes({needle_lit})) {{
                                window.__codetether_seen_text[{needle_lit}] = true;
                            }}
                        }} catch (_) {{ /* ignore */ }}
                    }});
                    obs.observe(document.body, {{ childList: true, subtree: true, characterData: true }});
                    window[key] = obs;
                }}
                return false;
            }})()"
        );
        let _ = page.evaluate(install.as_str()).await;
    }
    let probe = format!(
        "(() => {{
            const dq = ({dq_install});
            const seen = (window.__codetether_seen_text || {{}})[{needle_lit}] === true;
            if (seen) return true;
            const root = {scope_lit} ? dq({scope_lit}) : document.body;
            if (!root) return false;
            const t = (root.innerText || root.textContent || '');
            return t.includes({needle_lit});
        }})()"
    );
    loop {
        let present = page
            .evaluate(probe.as_str())
            .await?
            .into_value::<bool>()
            .unwrap_or(false);
        if present == want_present {
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::WaitTimeout {
                selector: format!("text={needle:?}"),
                timeout_ms,
            });
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_url(
    page: &Page,
    needle: &str,
    deadline: Instant,
    timeout_ms: u64,
) -> Result<BrowserOutput, BrowserError> {
    loop {
        let url = page.url().await?.unwrap_or_default();
        if url.contains(needle) {
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::WaitTimeout {
                selector: format!("url~={needle:?}"),
                timeout_ms,
            });
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Block until there has been no in-flight `fetch` / `XMLHttpRequest`
/// activity for at least 500 ms. React / Next.js / SvelteKit apps render
/// skeletons on first paint and then swap in real content after the API
/// call resolves; a plain `domcontentloaded` wait returns *before* that
/// swap, so the agent sees stale placeholder text.
///
/// Implementation detail: we install (once per page) a tiny in-page
/// counter that wraps `fetch` and `XMLHttpRequest.send`. The observer is
/// installed as early as possible, but if the page already has in-flight
/// requests when this wait starts we still wait for the counter to drop
/// to zero and stay zero for the quiet window.
async fn wait_network_idle(
    page: &Page,
    deadline: Instant,
    timeout_ms: u64,
) -> Result<BrowserOutput, BrowserError> {
    const QUIET_MS: u64 = 500;
    let install = r#"
        (() => {
            if (window.__codetether_netidle_installed) return true;
            window.__codetether_netidle_installed = true;
            window.__codetether_in_flight = 0;
            window.__codetether_last_change = Date.now();
            const bump = (delta) => {
                window.__codetether_in_flight = Math.max(0, window.__codetether_in_flight + delta);
                window.__codetether_last_change = Date.now();
            };
            const origFetch = window.fetch;
            if (origFetch) {
                window.fetch = function(...args) {
                    bump(1);
                    return origFetch.apply(this, args).finally(() => bump(-1));
                };
            }
            const XHR = window.XMLHttpRequest;
            if (XHR && XHR.prototype && XHR.prototype.send) {
                const origSend = XHR.prototype.send;
                XHR.prototype.send = function(...args) {
                    bump(1);
                    const done = () => bump(-1);
                    this.addEventListener('loadend', done, { once: true });
                    this.addEventListener('abort', done, { once: true });
                    this.addEventListener('error', done, { once: true });
                    return origSend.apply(this, args);
                };
            }
            return true;
        })()
    "#;
    let _ = page.evaluate(install).await;
    let probe = format!(
        "(() => ({{
            in_flight: window.__codetether_in_flight || 0,
            quiet_ms: Date.now() - (window.__codetether_last_change || Date.now())
        }}))()"
    );
    loop {
        let snapshot = page
            .evaluate(probe.as_str())
            .await?
            .into_value::<serde_json::Value>()
            .unwrap_or_default();
        let in_flight = snapshot
            .get("in_flight")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let quiet_ms = snapshot
            .get("quiet_ms")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        if in_flight <= 0 && quiet_ms >= QUIET_MS as i64 {
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::WaitTimeout {
                selector: "network_idle".into(),
                timeout_ms,
            });
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_ready_state(
    page: &Page,
    accept: &[&str],
    deadline: Instant,
    timeout_ms: u64,
) -> Result<BrowserOutput, BrowserError> {
    loop {
        let state = page
            .evaluate("document.readyState")
            .await?
            .into_value::<String>()
            .unwrap_or_default();
        if accept.iter().any(|want| state == *want) {
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::WaitTimeout {
                selector: format!("readyState in {accept:?}"),
                timeout_ms,
            });
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
