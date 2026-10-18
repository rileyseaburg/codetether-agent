use super::{Action, Host};
use codetether_tui_models::ViewMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Select a modifier shortcut after giving scrolling and approvals priority.
/// # Arguments
/// * `host` - Current state and native effect bindings.
/// * `key` - Terminal key event.
/// # Returns
/// None to fall through, or a consumed/quit result from the selected handler.
/// # Errors
/// Propagates errors from native scrolling or fallback handlers.
/// # Examples
/// ```text
/// modified::dispatch(&mut native_host, key)
/// ```
pub fn dispatch<H: Host>(host: &mut H, key: KeyEvent) -> Option<Result<bool, H::Error>> {
    if let Some(r) = host.alt_scroll(key) {
        return Some(r);
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char(character) if ctrl && host.approval(character) => {}
        KeyCode::Char('c') if ctrl => return Some(Ok(host.ctrl_c())),
        KeyCode::Char('q') if ctrl => return Some(Ok(true)),
        KeyCode::Char('t') if ctrl => {
            host.effect(Action::SymbolSearch);
            *host.status() = "Symbol search".to_string();
        }
        KeyCode::Char('w') if ctrl && host.in_chat() => {
            host.effect(Action::SideQuestion);
        }
        KeyCode::Char('b') if ctrl && host.in_chat() => {
            host.effect(Action::ToggleLayout);
            let label = if host.webview() { "Webview" } else { "Classic" };
            *host.status() = format!("Layout: {label}");
        }
        KeyCode::Char('l') if ctrl && host.in_chat() => {
            host.effect(Action::ClearMessages);
            host.effect(Action::ClearStreaming);
            *host.status() = "Screen cleared".to_string();
        }
        KeyCode::Char('s') if ctrl && host.in_chat() => {
            host.effect(Action::SaveScroll);
            host.effect(Action::View(ViewMode::Swarm));
            *host.status() = "Swarm".to_string();
        }
        KeyCode::Char('p') if ctrl && host.in_chat() => {
            host.effect(Action::SaveScroll);
            host.effect(Action::View(ViewMode::Protocol));
            *host.status() = "Protocol registry".to_string();
        }
        KeyCode::Char('o') if ctrl && host.in_chat() => {
            host.effect(Action::CopyReply);
        }
        KeyCode::Char('r') if ctrl && host.in_chat() => {
            host.effect(Action::Voice);
        }
        KeyCode::Char('v') if ctrl && host.in_chat() => {
            host.effect(Action::Paste);
        }
        KeyCode::Insert if key.modifiers.contains(KeyModifiers::SHIFT) && host.in_chat() => {
            // Shift+Insert is the standard paste shortcut on Windows
            // and many Linux terminal emulators.
            host.effect(Action::Paste);
        }
        KeyCode::Char('Y') if ctrl && host.in_chat() => {
            // Kitty keyboard protocol: Ctrl+Shift+Y arrives as 'Y' + CONTROL.
            host.effect(Action::CopyTranscript);
        }
        KeyCode::Char('y') if ctrl && host.in_chat() => {
            // Legacy terminals: Ctrl+Shift+Y arrives as 'y' + CONTROL+SHIFT.
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                host.effect(Action::CopyTranscript);
            } else {
                host.effect(Action::CopyReply);
            }
        }
        KeyCode::Char('x') if ctrl && host.watchdog_active() => {
            host.effect(Action::CancelWatchdog);
        }
        _ => return host.fallback(key, ctrl),
    }
    Some(Ok(false))
}
