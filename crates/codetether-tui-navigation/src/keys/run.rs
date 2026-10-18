use super::Host;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Dispatch a key after modifier shortcuts have been processed.
/// # Arguments
/// * `host` - Native input and navigation bindings.
/// * `key` - Key event to route.
/// # Returns
/// False; quit requests are handled by the preceding modifier layer.
/// # Examples
/// ```text
/// keys::dispatch(&mut native_host, key).await;
/// ```
pub async fn dispatch(host: &mut impl Host, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => escape(host),
        KeyCode::Tab | KeyCode::BackTab => host.tab(key.code),
        KeyCode::Char('?') if !host.in_chat() => crate::toggle_help(&mut host.navigation()),
        KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            host.model_picker().await
        }
        KeyCode::Up => crate::handle_up(&mut host.navigation(), key.modifiers),
        KeyCode::Down => crate::handle_down(&mut host.navigation(), key.modifiers),
        KeyCode::PageUp => crate::handle_page_up(&mut host.navigation()),
        KeyCode::PageDown => crate::handle_page_down(&mut host.navigation()),
        KeyCode::Home => crate::handle_home(&mut host.navigation()),
        KeyCode::End => crate::handle_end(&mut host.navigation()),
        KeyCode::Left => crate::handle_left(&mut host.navigation(), key.modifiers),
        KeyCode::Right => crate::handle_right(&mut host.navigation(), key.modifiers),
        KeyCode::Delete => crate::handle_delete(&mut host.navigation()),
        KeyCode::Enter if host.symbols_active() => host.symbol_enter(),
        // Shift+Enter / Alt+Enter inserts a literal newline into the
        // chat input instead of submitting. Must come BEFORE the bare
        // `KeyCode::Enter` arm. Shift+Enter requires the terminal to
        // report modifier bits on Enter (kitty keyboard protocol or
        // xterm modifyOtherKeys — see `PushKeyboardEnhancementFlags`
        // in run.rs); Alt+Enter is universally distinguishable.
        KeyCode::Enter
            if key.modifiers.contains(KeyModifiers::SHIFT)
                || key.modifiers.contains(KeyModifiers::ALT) =>
        {
            host.newline();
        }
        KeyCode::Enter => {
            if host.paste_newline() {
                host.newline();
            } else {
                host.enter().await;
            }
        }
        _ => host.other(key).await,
    }
    false
}
fn escape(host: &mut impl Host) {
    if host.watchdog_active() {
        host.dismiss_watchdog();
    } else {
        crate::handle_escape(&mut host.navigation());
    }
}
