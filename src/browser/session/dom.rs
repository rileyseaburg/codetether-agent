use super::{BrowserSession, access, device, humanize};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::{Ack, HtmlContent, TextContent},
    request::{FillRequest, ScopeRequest, SelectorRequest},
};
use chromiumoxide::{element::Element, error::CdpError, page::Page};
use serde::Deserialize;

#[derive(Deserialize)]
struct FillTarget {
    fillable: bool,
    found: bool,
    input_type: Option<String>,
    tag: String,
}

#[derive(Deserialize)]
struct ClickPoint {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

pub(super) async fn click(
    session: &BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    ensure_page_scope(request.frame_selector.as_deref())?;
    let page = access::current_page(session).await?;
    ensure_present(&page, &request.selector).await?;
    // Scroll into view before clicking: React apps often render off-screen
    // rows inside virtualized lists or below-the-fold panels, and CDP's
    // `Element::click` will fail with "click point not in viewport" on a
    // fresh navigation even though the selector resolves. A best-effort
    // scroll is cheap and removes a whole class of spurious failures.
    let _ = page
        .evaluate(format!(
            "(() => {{ const el = document.querySelector({lit}); if (el) el.scrollIntoView({{block:'center', inline:'center'}}); return true; }})()",
            lit = serde_json::to_string(&request.selector)?
        ))
        .await;
    // Humanized click path: compute the element's viewport bounding box,
    // pick a jittered point inside it, then move the mouse to the point
    // and press/release. Real users never click at the exact pixel-center
    // of every element, and they always move the pointer before clicking.
    // Falls back to chromiumoxide's `Element::click` (center + no move) on
    // any error so we don't regress when rect lookup fails.
    match click_point_for(&page, &request.selector).await {
        Ok(Some(p)) => {
            let radius = (p.w.min(p.h) / 4.0).min(8.0).max(0.0);
            let (x, y) = humanize::jitter_point(p.x, p.y, radius);
            if let Err(err) = device::human_click_at(&page, x, y).await {
                if is_intercepted(&err) {
                    // One retry after a short settle — overlay may be mid-animation.
                    humanize::settle_delay().await;
                    device::human_click_at(&page, x, y).await?;
                } else {
                    return Err(err);
                }
            }
        }
        _ => {
            let element = resolve_element(&page, &request.selector).await?;
            if let Err(error) = element.click().await {
                if matches!(error, CdpError::NotFound) || is_intercepted_cdp(&error) {
                    let element = resolve_element(&page, &request.selector).await?;
                    element.click().await?;
                } else {
                    return Err(error.into());
                }
            }
        }
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Resolve the click point for `selector`: viewport-space centre of the
/// first client rect, plus half-extents so the caller can jitter inside
/// the element. Returns `Ok(None)` when the element exists but has no
/// layout (zero-sized, display:none, detached).
async fn click_point_for(
    page: &Page,
    selector: &str,
) -> Result<Option<ClickPoint>, BrowserError> {
    let selector_lit = serde_json::to_string(selector)?;
    let script = format!(
        "(() => {{
            const el = document.querySelector({selector_lit});
            if (!el) return null;
            const r = el.getBoundingClientRect();
            if (!r || r.width === 0 || r.height === 0) return null;
            return {{ x: r.left + r.width / 2, y: r.top + r.height / 2, w: r.width, h: r.height }};
        }})()"
    );
    let value = page.evaluate(script).await?;
    Ok(value.into_value::<Option<ClickPoint>>().unwrap_or(None))
}

/// Did a [`BrowserError`] from the humanized click path come from an
/// intercepting overlay? Mirrors [`is_intercepted_cdp`] but over the
/// wrapped error type.
fn is_intercepted(error: &BrowserError) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    msg.contains("intercept") || msg.contains("not clickable") || msg.contains("obscured")
}

/// Did the click fail because another element (usually an animation overlay
/// or a newly-mounted modal) intercepted the pointer event? CDP reports this
/// as a JavaScript exception from the click script rather than a typed
/// variant, so we pattern-match on the message.
fn is_intercepted_cdp(error: &CdpError) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    msg.contains("intercept") || msg.contains("not clickable") || msg.contains("obscured")
}

pub(super) async fn fill(
    session: &BrowserSession,
    request: FillRequest,
) -> Result<BrowserOutput, BrowserError> {
    ensure_page_scope(request.frame_selector.as_deref())?;
    let page = access::current_page(session).await?;
    ensure_fillable(&page, &request.selector).await?;
    let element = resolve_element(&page, &request.selector).await?;
    clear_value(&element).await?;
    element.focus().await?;
    humanize::settle_delay().await;
    // Type character-by-character with human-plausible pacing so per-key
    // handlers (validation, autocomplete, fraud detection) see the input as
    // keystrokes rather than a single burst.
    for ch in request.value.chars() {
        element.type_str(ch.to_string()).await?;
        humanize::keystroke_delay().await;
    }
    // React & Vue controlled inputs keep their own value tracker that is
    // only updated by *native* value setters. Plain `type_str` dispatches
    // key events, which fills the DOM `value` attribute but leaves the
    // framework's tracker out of sync — so the next render reverts the
    // field. Poking the native setter AFTER the keystrokes and dispatching
    // `input` + `change` lets frameworks pick up the new value while still
    // running any onKeyDown handlers a pure-setter approach would bypass.
    let selector_lit = serde_json::to_string(&request.selector)?;
    let value_lit = serde_json::to_string(&request.value)?;
    let script = format!(
        "(() => {{
            const el = document.querySelector({selector_lit});
            if (!el) return false;
            const proto = Object.getPrototypeOf(el);
            const setter = Object.getOwnPropertyDescriptor(proto, 'value');
            if (setter && setter.set && el.value !== {value_lit}) {{
                setter.set.call(el, {value_lit});
            }}
            el.dispatchEvent(new Event('input', {{ bubbles: true }}));
            el.dispatchEvent(new Event('change', {{ bubbles: true }}));
            return true;
        }})()"
    );
    let _ = page.evaluate(script).await;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

pub(super) async fn text(
    session: &BrowserSession,
    request: ScopeRequest,
) -> Result<BrowserOutput, BrowserError> {
    ensure_page_scope(request.frame_selector.as_deref())?;
    let text = match request.selector {
        Some(selector) => resolve_element(&access::current_page(session).await?, &selector)
            .await?
            .inner_text()
            .await?
            .unwrap_or_default(),
        None => access::current_page(session)
            .await?
            .evaluate("document.body ? document.body.innerText : ''")
            .await?
            .into_value()?,
    };
    Ok(BrowserOutput::Text(TextContent { text }))
}

pub(super) async fn html(
    session: &BrowserSession,
    request: ScopeRequest,
) -> Result<BrowserOutput, BrowserError> {
    ensure_page_scope(request.frame_selector.as_deref())?;
    let html = match request.selector {
        Some(selector) => resolve_element(&access::current_page(session).await?, &selector)
            .await?
            .outer_html()
            .await?
            .unwrap_or_default(),
        None => access::current_page(session).await?.content().await?,
    };
    Ok(BrowserOutput::Html(HtmlContent { html }))
}

async fn resolve_element(page: &Page, selector: &str) -> Result<Element, BrowserError> {
    match page.find_element(selector).await {
        Ok(element) => Ok(element),
        Err(error) if stale_node(&error) => page
            .find_element(selector)
            .await
            .map_err(|retry| element_error(retry, selector)),
        Err(error) => Err(element_error(error, selector)),
    }
}

async fn ensure_fillable(page: &Page, selector: &str) -> Result<(), BrowserError> {
    let target = inspect_fill_target(page, selector).await?;
    if !target.found {
        return Err(BrowserError::ElementNotFound(selector.to_string()));
    }
    if target.fillable {
        return Ok(());
    }
    Err(BrowserError::ElementNotFillable {
        tag: target.tag,
        input_type: target.input_type,
    })
}

async fn clear_value(element: &Element) -> Result<(), BrowserError> {
    element
        .call_js_fn(
            "function() {
                if ('value' in this) {
                    this.value = '';
                    this.dispatchEvent(new Event('input', { bubbles: true }));
                }
            }",
            true,
        )
        .await?;
    Ok(())
}

fn ensure_page_scope(frame_selector: Option<&str>) -> Result<(), BrowserError> {
    if frame_selector.is_some_and(|value| !value.trim().is_empty()) {
        return Err(BrowserError::OperationFailed(
            "frame-scoped DOM actions are not implemented yet".into(),
        ));
    }
    Ok(())
}

fn element_error(error: CdpError, selector: &str) -> BrowserError {
    match error {
        CdpError::NotFound => BrowserError::ElementNotFound(selector.to_string()),
        other => other.into(),
    }
}

fn stale_node(error: &CdpError) -> bool {
    matches!(error, CdpError::ChromeMessage(message) if message.contains("Could not find node with given id"))
}

async fn ensure_present(page: &Page, selector: &str) -> Result<(), BrowserError> {
    let encoded = serde_json::to_string(selector)?;
    let script = format!("(() => Boolean(document.querySelector({encoded})))()");
    let found: bool = page.evaluate_expression(script).await?.into_value()?;
    if found {
        return Ok(());
    }
    Err(BrowserError::ElementNotFound(selector.to_string()))
}

async fn inspect_fill_target(page: &Page, selector: &str) -> Result<FillTarget, BrowserError> {
    let selector = serde_json::to_string(selector)?;
    let script = format!(
        "(() => {{
            const el = document.querySelector({selector});
            if (!el) {{
                return {{ found: false, tag: '', input_type: null, fillable: false }};
            }}
            const tag = (el.tagName || '').toLowerCase();
            if (tag === 'textarea') {{
                return {{ found: true, tag, input_type: null, fillable: true }};
            }}
            if (tag !== 'input') {{
                return {{ found: true, tag, input_type: null, fillable: false }};
            }}
            const inputType = (el.type || 'text').toLowerCase();
            const blocked = ['checkbox','radio','file','submit','button','image','reset'];
            return {{
                found: true,
                tag,
                input_type: inputType,
                fillable: !blocked.includes(inputType)
            }};
        }})()"
    );
    Ok(page.evaluate_expression(script).await?.into_value()?)
}
