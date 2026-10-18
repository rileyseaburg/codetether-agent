use super::{Action, Host};
use codetether_tui_models::{InputMode, ViewMode};
use crossterm::event::KeyModifiers;
/// Route a character to the active search, filter or chat buffer.
/// # Arguments
/// * `host` - Current input state and native mutation bindings.
/// * `modifiers`, `c` - Modifier flags and inserted character.
/// # Returns
/// Applies the selected input mutation and any corresponding mode/status update.
/// # Examples
/// ```text
/// text::character(&mut native_host, modifiers, c);
/// ```
pub fn character(host: &mut impl Host, modifiers: KeyModifiers, c: char) {
    let no_mods =
        !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT);

    if no_mods && host.symbols_active() {
        // Whitespace ends the mention: the already-typed query stays in the
        // chat buffer so nothing the user typed is silently discarded.
        if c.is_whitespace() {
            host.effect(Action::DismissMention);
            host.effect(Action::Insert(c));
            return;
        }
        host.effect(Action::SymbolChar(c));
        host.effect(Action::RefreshSymbols);
    } else if host.in_view(ViewMode::Bus) && host.bus_filter_active() && no_mods {
        host.effect(Action::BusPush(c));
        let status = format!("Protocol filter: {}", host.bus_filter());
        host.set_status(status);
    } else if host.in_view(ViewMode::Model) && no_mods {
        host.effect(Action::ModelPush(c));
    } else if host.in_view(ViewMode::FilePicker) && no_mods {
        host.effect(Action::FilePush(c));
    } else if host.in_view(ViewMode::Chat) && no_mods {
        if c == '@' {
            // open() inserts the '@' itself in both the mention and the
            // literal case, so there is nothing further to insert here.
            host.effect(Action::OpenMention);
            return;
        }
        let mode = if host.input().is_empty() && c == '/' {
            InputMode::Command
        } else if host.input().starts_with('/') || c == '/' {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        host.set_mode(mode);
        host.effect(Action::Insert(c));
    }
}
