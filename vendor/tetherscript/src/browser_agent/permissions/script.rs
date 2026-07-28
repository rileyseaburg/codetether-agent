//! JavaScript bridge snippets for permission emulation.

mod base;
mod clipboard;
mod geolocation;
mod media;
mod notification;

use super::{BrowserPermission, GeolocationEmulation, PermissionState};
use crate::browser_agent::keyboard_escape::quote;

pub(crate) fn install(
    states: &[(BrowserPermission, PermissionState)],
    geolocation: &GeolocationEmulation,
    clipboard: &str,
) -> String {
    format!(
        "var __states={};var __geo={};window.__agentClipboardText={};{}",
        states_object(states),
        geolocation::object(geolocation),
        quote(clipboard),
        bridge_source()
    )
}

pub(crate) fn drain() -> &'static str {
    "window.__agentClipboardText||'';"
}

fn states_object(states: &[(BrowserPermission, PermissionState)]) -> String {
    let body = states
        .iter()
        .map(|(name, state)| format!("{}:{}", quote(name.name()), quote(state.as_str())))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{body}}}")
}

fn bridge_source() -> String {
    let mut source = String::from(base::SOURCE);
    source.push_str(&clipboard::source());
    source.push_str(media::SOURCE);
    source.push_str(notification::SOURCE);
    source
}
