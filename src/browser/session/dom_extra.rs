//! Extra DOM actions: text-based clicking, form helpers, sequenced typing,
//! single-key press, and toggle for checkboxes/radios.
//!
//! These complement [`super::dom`] which covers the basic CSS-selector based
//! interactions. The actions here all share a common pattern: locate an
//! element via either an explicit selector or a text-search inside a scope,
//! then drive it through chromiumoxide's high-level helpers (`click()`,
//! `type_str()`, `press_key()`).

use super::{BrowserSession, access, humanize};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::Ack,
    request::{ClickTextRequest, FillRequest, KeyPressRequest, ToggleRequest, TypeRequest},
};
use chromiumoxide::page::Page;
use std::time::Duration;
use tokio::time::{Instant, sleep};

/// Type a string into an element character-by-character with optional delay.
///
/// Differs from [`super::dom::fill`] in three ways: (1) appends instead of
/// replacing, (2) dispatches per-key events so JS frameworks see the input as
/// keystrokes rather than a programmatic value change, (3) honors `delay_ms`
/// to throttle the typing rate (useful for slow IME/autocomplete handlers).
pub(super) async fn type_text(
    session: &BrowserSession,
    request: TypeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let element = page.find_element(&request.selector).await?;
    element.focus().await?;
    humanize::settle_delay().await;
    // Type one character at a time so per-keystroke JS handlers fire in
    // order, with a human-plausible delay between keys. If the caller
    // specified an explicit `delay_ms`, that takes precedence; otherwise we
    // use a randomized human-like cadence.
    for ch in request.text.chars() {
        element.type_str(ch.to_string()).await?;
        if request.delay_ms == 0 {
            humanize::keystroke_delay().await;
        } else {
            sleep(Duration::from_millis(request.delay_ms)).await;
        }
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Send a single key press to an element (e.g. `Enter`, `Tab`, `ArrowDown`).
///
/// Uses the W3C key name; chromiumoxide maps these to CDP `dispatchKeyEvent`.
pub(super) async fn press(
    session: &BrowserSession,
    request: KeyPressRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let element = page.find_element(&request.selector).await?;
    element.focus().await?.press_key(&request.key).await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Click the first element whose visible text matches `text`.
///
/// Polls every 100 ms up to `timeout_ms`. If `selector` is provided, the
/// search is scoped to descendants of that selector; otherwise the whole
/// document is searched. `exact=true` matches the trimmed text exactly,
/// `exact=false` does a case-insensitive substring match. `index` selects
/// the Nth match (zero-based) when multiple elements match.
pub(super) async fn click_text(
    session: &BrowserSession,
    request: ClickTextRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let deadline = Instant::now() + Duration::from_millis(request.timeout_ms);
    loop {
        if locate_text(&page, &request).await? {
            click_marked(&page).await?;
            return Ok(BrowserOutput::Ack(Ack { ok: true }));
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::ElementNotFound(format!(
                "no element with text {:?}",
                request.text
            )));
        }
        sleep(Duration::from_millis(100)).await;
    }
}

/// Toggle a checkbox or radio input identified by `selector`. If the element
/// is currently unchecked it becomes checked and vice versa, dispatching the
/// appropriate `change` event so framework state updates.
pub(super) async fn toggle(
    session: &BrowserSession,
    request: ToggleRequest,
) -> Result<BrowserOutput, BrowserError> {
    let _ = request.text;
    let _ = request.timeout_ms;
    let page = access::current_page(session).await?;
    let selector_lit = serde_json::to_string(&request.selector)?;
    let script = format!(
        "(() => {{
            const el = document.querySelector({selector_lit});
            if (!el) return false;
            if (el.tagName === 'INPUT' && (el.type === 'checkbox' || el.type === 'radio')) {{
                el.checked = !el.checked;
                el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                return true;
            }}
            el.click();
            return true;
        }})()"
    );
    let ok: bool = page.evaluate(script).await?.into_value()?;
    if !ok {
        return Err(BrowserError::ElementNotFound(request.selector));
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Set a value on an input/textarea by writing the property directly and
/// dispatching `input` + `change` events. Faster than [`super::dom::fill`] for
/// long strings and works on inputs whose JS framework intercepts every
/// keystroke (where character-by-character typing causes flickering).
pub(super) async fn fill_native(
    session: &BrowserSession,
    request: FillRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let selector_lit = serde_json::to_string(&request.selector)?;
    let value_lit = serde_json::to_string(&request.value)?;
    let script = format!(
        "(() => {{
            const el = document.querySelector({selector_lit});
            if (!el) return false;
            const proto = Object.getPrototypeOf(el);
            const setter = Object.getOwnPropertyDescriptor(proto, 'value');
            if (setter && setter.set) {{
                setter.set.call(el, {value_lit});
            }} else {{
                el.value = {value_lit};
            }}
            el.dispatchEvent(new Event('input', {{ bubbles: true }}));
            el.dispatchEvent(new Event('change', {{ bubbles: true }}));
            return true;
        }})()"
    );
    let ok: bool = page.evaluate(script).await?.into_value()?;
    if !ok {
        return Err(BrowserError::ElementNotFound(request.selector));
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Find the Nth element matching the text constraints and tag it with a
/// transient `data-codetether-click-target` attribute. Returns `true` when
/// a match was tagged.
///
/// We do the matching in injected JS rather than walking the DOM via CDP
/// because a tree walk over a large page is hundreds of round-trips, while a
/// single `evaluate` is one round-trip plus tens of ms of JS.
async fn locate_text(page: &Page, req: &ClickTextRequest) -> Result<bool, BrowserError> {
    let scope_lit = serde_json::to_string(req.selector.as_deref().unwrap_or(""))?;
    let text_lit = serde_json::to_string(&req.text)?;
    let exact = req.exact;
    let index = req.index;
    let script = format!(
        "(() => {{
            const scope = {scope_lit} ? document.querySelector({scope_lit}) : document;
            if (!scope) return false;
            const want = {text_lit};
            const exact = {exact};
            const wantLower = want.toLowerCase();
            const matches = [];
            const candidates = scope.querySelectorAll('a, button, [role=\"button\"], [role=\"link\"], [role=\"tab\"], [role=\"menuitem\"], summary, label, [onclick]');
            for (const el of candidates) {{
                const txt = (el.innerText || el.textContent || '').trim();
                const ok = exact ? txt === want : txt.toLowerCase().includes(wantLower);
                if (ok) matches.push(el);
            }}
            if (matches.length === 0) {{
                const walker = document.createTreeWalker(scope, NodeFilter.SHOW_ELEMENT);
                while (walker.nextNode()) {{
                    const el = walker.currentNode;
                    const txt = (el.innerText || '').trim();
                    if (!txt) continue;
                    const ok = exact ? txt === want : txt.toLowerCase().includes(wantLower);
                    if (ok) matches.push(el);
                }}
            }}
            const target = matches[{index}];
            if (!target) return false;
            target.scrollIntoView({{ block: 'center', inline: 'center' }});
            document.querySelectorAll('[data-codetether-click-target]').forEach(el => el.removeAttribute('data-codetether-click-target'));
            target.setAttribute('data-codetether-click-target', '1');
            return true;
        }})()"
    );
    Ok(page.evaluate(script).await?.into_value::<bool>().unwrap_or(false))
}

/// Click the element previously tagged by [`locate_text`]. Re-querying via
/// CDP gives us a real `Element` handle so the click goes through the
/// normal input pipeline (gesture detection, framework listeners) rather
/// than a synthetic `el.click()` JS dispatch.
async fn click_marked(page: &Page) -> Result<(), BrowserError> {
    let element = page.find_element("[data-codetether-click-target=\"1\"]").await?;
    element.click().await?;
    let _ = page
        .evaluate(
            "document.querySelectorAll('[data-codetether-click-target]').forEach(el => el.removeAttribute('data-codetether-click-target'))",
        )
        .await;
    Ok(())
}
