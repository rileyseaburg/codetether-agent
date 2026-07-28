//! WebGL metadata attribute parser.

use crate::browser::Element;

use super::canvas_webgl_commands::parse_command;
use super::canvas_webgl_model::{WebGlCommand, WebGlContextSnapshot};

pub(super) fn snapshot_from_element(element: &Element) -> Option<WebGlContextSnapshot> {
    Some(WebGlContextSnapshot {
        version: attr_u8(element, "data-agent-webgl-version")?,
        width: attr_u32(element, "data-agent-webgl-width")
            .or_else(|| attr_u32(element, "width"))
            .unwrap_or(300),
        height: attr_u32(element, "data-agent-webgl-height")
            .or_else(|| attr_u32(element, "height"))
            .unwrap_or(150),
        viewport: attr_array(element, "data-agent-webgl-viewport", 0),
        clear_color: attr_array(element, "data-agent-webgl-clear-color", 0.0),
        supported_extensions: attr_list(element, "data-agent-webgl-extensions"),
        commands: commands_from_attr(element),
    })
}

fn attr_u8(element: &Element, name: &str) -> Option<u8> {
    element.attrs.get(name).and_then(|value| value.parse().ok())
}

fn attr_u32(element: &Element, name: &str) -> Option<u32> {
    element.attrs.get(name).and_then(|value| value.parse().ok())
}

fn attr_array<T: Copy + std::str::FromStr>(element: &Element, name: &str, default: T) -> [T; 4] {
    let mut out = [default; 4];
    if let Some(raw) = element.attrs.get(name) {
        for (idx, part) in raw.split(',').take(4).enumerate() {
            out[idx] = part.parse().unwrap_or(default);
        }
    }
    out
}

fn attr_list(element: &Element, name: &str) -> Vec<String> {
    element
        .attrs
        .get(name)
        .map(|raw| raw.split(';').map(str::to_string).collect())
        .unwrap_or_default()
}

fn commands_from_attr(element: &Element) -> Vec<WebGlCommand> {
    element
        .attrs
        .get("data-agent-webgl-commands")
        .map(|raw| raw.split(';').filter_map(parse_command).collect())
        .unwrap_or_default()
}
