use super::{Action, Host};
use codetether_tui_models::{InputMode, ViewMode};

/// Route backspace to the active search, filter or chat buffer.
/// # Arguments
/// * `host` - Current input state and native mutation bindings.

/// # Returns
/// Applies the selected input mutation and any corresponding mode/status update.
/// # Examples
/// ```text
/// text::backspace(&mut native_host);
/// ```
pub fn backspace(host: &mut impl Host) {
    if host.symbols_active() {
        host.effect(Action::SymbolBackspace);
        host.effect(Action::RefreshSymbols);
    } else if host.in_view(ViewMode::Bus) && host.bus_filter_active() {
        host.effect(Action::BusPop);
        let status = if host.bus_filter().is_empty() {
            "Protocol filter cleared".to_string()
        } else {
            format!("Protocol filter: {}", host.bus_filter())
        };
        host.set_status(status);
    } else if host.in_view(ViewMode::Model) {
        host.effect(Action::ModelBackspace);
    } else if host.in_view(ViewMode::FilePicker) {
        host.effect(Action::FileBackspace);
    } else if host.in_view(ViewMode::Chat) {
        host.effect(Action::DeleteBackspace);
        if host.input().is_empty() {
            let mode = InputMode::Normal;
            host.set_mode(mode);
        } else if host.input().starts_with('/') {
            let mode = InputMode::Command;
            host.set_mode(mode);
        }
    }
}
