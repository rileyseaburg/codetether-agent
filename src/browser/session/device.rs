//! Device-level input: raw mouse and keyboard events that are not bound to
//! a specific selector. These mirror what a human user would do via the
//! physical mouse/keyboard and bypass any element-resolution logic.
//!
//! The mouse helper takes absolute viewport coordinates and dispatches a
//! synthetic press+release pair via CDP `Input.dispatchMouseEvent`. The
//! keyboard helpers operate on whichever element currently has focus.

use super::{BrowserSession, access, humanize};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::Ack,
    request::{KeyboardPressRequest, KeyboardTypeRequest, PointerClick},
};
use chromiumoxide::cdp::browser_protocol::input::{
    DispatchKeyEventParams, DispatchKeyEventType, DispatchMouseEventParams, DispatchMouseEventType,
    MouseButton,
};
use chromiumoxide::page::Page;

/// Pointer-state tracker (per-process). Real mice don't teleport, so we
/// remember where the pointer last was and move it along a short
/// trajectory toward the next click target.
static POINTER_X: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static POINTER_Y: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn last_pointer() -> (f64, f64) {
    let xb = POINTER_X.load(std::sync::atomic::Ordering::Relaxed);
    let yb = POINTER_Y.load(std::sync::atomic::Ordering::Relaxed);
    (f64::from_bits(xb), f64::from_bits(yb))
}

fn remember_pointer(x: f64, y: f64) {
    POINTER_X.store(x.to_bits(), std::sync::atomic::Ordering::Relaxed);
    POINTER_Y.store(y.to_bits(), std::sync::atomic::Ordering::Relaxed);
}

/// Move the mouse from the last known position to `(x, y)` via a handful
/// of intermediate `MouseMoved` events. Bot detectors look for clicks
/// with no preceding move events; a short eased trajectory defeats that.
pub(super) async fn human_move_to(page: &Page, x: f64, y: f64) -> Result<(), BrowserError> {
    let (fx, fy) = last_pointer();
    let dx = x - fx;
    let dy = y - fy;
    let dist = (dx * dx + dy * dy).sqrt();
    // Steps scale with distance (roughly one step per 40 px, capped at 12).
    // First move of the session (dist == 0 because pointer starts at origin)
    // still emits one mousemove so the target page sees pointer activity.
    let steps = ((dist / 40.0).ceil() as usize).clamp(3, 12);
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        // Ease-out cubic for a natural deceleration.
        let eased = 1.0 - (1.0 - t).powi(3);
        let (jx, jy) = humanize::jitter_point(fx + dx * eased, fy + dy * eased, 1.5);
        page.execute(
            DispatchMouseEventParams::builder()
                .x(jx)
                .y(jy)
                .r#type(DispatchMouseEventType::MouseMoved)
                .build()
                .map_err(BrowserError::OperationFailed)?,
        )
        .await?;
    }
    remember_pointer(x, y);
    Ok(())
}

/// Full human click at `(x, y)`: move to the point, settle briefly, press,
/// hold, release. Shared by [`mouse_click`] and the element-based click
/// path in [`super::dom`].
pub(super) async fn human_click_at(page: &Page, x: f64, y: f64) -> Result<(), BrowserError> {
    human_move_to(page, x, y).await?;
    humanize::settle_delay().await;
    press_release(page, x, y).await
}

async fn press_release(page: &Page, x: f64, y: f64) -> Result<(), BrowserError> {
    page.execute(
        DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .click_count(1)
            .r#type(DispatchMouseEventType::MousePressed)
            .build()
            .map_err(BrowserError::OperationFailed)?,
    )
    .await?;
    humanize::click_hold_delay().await;
    page.execute(
        DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .click_count(1)
            .r#type(DispatchMouseEventType::MouseReleased)
            .build()
            .map_err(BrowserError::OperationFailed)?,
    )
    .await?;
    Ok(())
}

/// Click at a precise viewport coordinate. Useful for canvas/WebGL UIs and
/// for hitting elements that escape selectors (PDF viewers, video controls).
pub(super) async fn mouse_click(
    session: &BrowserSession,
    request: PointerClick,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let (x, y) = humanize::jitter_point(request.x, request.y, 2.0);
    human_click_at(&page, x, y).await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Type a string into whatever element currently has keyboard focus.
/// Each character becomes its own `keyDown`/`char`/`keyUp` triple at the CDP
/// level so JS keypress handlers fire as if from a real keyboard.
pub(super) async fn keyboard_type(
    session: &BrowserSession,
    request: KeyboardTypeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    for ch in request.text.chars() {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf).to_string();
        page.execute(
            DispatchKeyEventParams::builder()
                .r#type(DispatchKeyEventType::Char)
                .text(s.clone())
                .unmodified_text(s)
                .build()
                .map_err(BrowserError::OperationFailed)?,
        )
        .await?;
        humanize::keystroke_delay().await;
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Press and release a single named key (e.g. `Enter`, `Tab`, `Escape`).
/// Routes through chromiumoxide's keyboard helper which has the W3C key→VK
/// map; any unknown key results in a no-op `Char` event.
pub(super) async fn keyboard_press(
    session: &BrowserSession,
    request: KeyboardPressRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let down = key_event(&request.key, DispatchKeyEventType::KeyDown)?;
    let up = key_event(&request.key, DispatchKeyEventType::KeyUp)?;
    page.execute(down).await?;
    page.execute(up).await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Build a `dispatchKeyEvent` for a named key. Single-character keys are
/// passed through as text so input fields receive them; named keys (Enter,
/// Tab, Escape, Backspace, arrows, function keys) are mapped to their W3C
/// `key` and DOM Level 3 `code` so framework handlers see the right values.
fn key_event(
    key: &str,
    kind: DispatchKeyEventType,
) -> Result<DispatchKeyEventParams, BrowserError> {
    let mut builder = DispatchKeyEventParams::builder().r#type(kind);
    let chars: Vec<char> = key.chars().collect();
    if chars.len() == 1 {
        // Single character: pass as `text` so the focused field accepts it.
        let c = chars[0];
        let s = c.to_string();
        builder = builder.key(s.clone()).text(s.clone()).unmodified_text(s);
    } else {
        // Named key: best-effort mapping. Browsers accept any string in `key`
        // but we set `windows_virtual_key_code` for the common ones so legacy
        // keypress handlers receive the expected `keyCode`.
        let code = match key {
            "Enter" | "Return" => Some((13_i64, "Enter")),
            "Tab" => Some((9, "Tab")),
            "Escape" | "Esc" => Some((27, "Escape")),
            "Backspace" => Some((8, "Backspace")),
            "Delete" => Some((46, "Delete")),
            "ArrowUp" => Some((38, "ArrowUp")),
            "ArrowDown" => Some((40, "ArrowDown")),
            "ArrowLeft" => Some((37, "ArrowLeft")),
            "ArrowRight" => Some((39, "ArrowRight")),
            "Home" => Some((36, "Home")),
            "End" => Some((35, "End")),
            "PageUp" => Some((33, "PageUp")),
            "PageDown" => Some((34, "PageDown")),
            "Space" | " " => Some((32, "Space")),
            _ => None,
        };
        builder = builder.key(key.to_string());
        if let Some((vk, code)) = code {
            builder = builder.windows_virtual_key_code(vk).code(code.to_string());
        }
    }
    builder.build().map_err(BrowserError::OperationFailed)
}
