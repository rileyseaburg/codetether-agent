use super::{Action, Host, Setting};
use codetether_tui_models::ViewMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Dispatch settings and mode-specific character/backspace keys.
/// # Arguments
/// * `host` - Current view and native input bindings.
/// * `key` - Key event not consumed by earlier dispatch layers.
/// # Returns
/// Completes after handling the key; unmatched keys have no effect.
/// # Examples
/// ```text
/// modes::dispatch(&mut native_host, key).await;
/// ```
pub async fn dispatch(host: &mut impl Host, key: KeyEvent) {
    match key.code {
        KeyCode::Char('a') if host.in_view(ViewMode::Settings) => {
            host.setting(Setting::AutoApply).await;
        }
        KeyCode::Char('a')
            if key.modifiers.contains(KeyModifiers::ALT) && host.in_view(ViewMode::FilePicker) =>
        {
            host.effect(Action::Attach);
        }
        KeyCode::Char('n') if host.in_view(ViewMode::Settings) => {
            host.setting(Setting::Network).await;
        }
        KeyCode::Tab if host.in_view(ViewMode::Settings) => {
            host.setting(Setting::Autocomplete).await;
        }
        KeyCode::Backspace if host.in_view(ViewMode::Sessions) => {
            host.effect(Action::FilterBackspace);
        }
        KeyCode::Backspace => host.backspace().await,
        KeyCode::Char('g') if host.in_view(ViewMode::Bus) => host.effect(Action::BusG),
        KeyCode::Char('c') if host.in_view(ViewMode::Bus) => host.effect(Action::BusC),
        KeyCode::Char('/') if host.in_view(ViewMode::Bus) => host.effect(Action::BusFilter),
        KeyCode::Char('f') if host.in_view(ViewMode::Audit) => {
            host.effect(Action::AuditFilter);
        }
        KeyCode::Char(c) if host.in_view(ViewMode::Sessions) => {
            host.session_char(key.modifiers, c);
        }
        KeyCode::Char(c) => host.character(key.modifiers, c).await,
        _ => {}
    }
}
