//! Canvas attribute parser.

use crate::browser::Element;

use super::canvas_model::{CanvasCommand, CanvasSurface};

pub(super) fn surface_from_element(element: &Element) -> CanvasSurface {
    CanvasSurface {
        width: attr_u32(element, "data-agent-canvas-width")
            .or_else(|| attr_u32(element, "width"))
            .unwrap_or(300),
        height: attr_u32(element, "data-agent-canvas-height")
            .or_else(|| attr_u32(element, "height"))
            .unwrap_or(150),
        commands: commands_from_attr(element),
        checksum: element
            .attrs
            .get("data-agent-canvas-checksum")
            .and_then(|value| value.parse().ok()),
    }
}

fn attr_u32(element: &Element, name: &str) -> Option<u32> {
    element.attrs.get(name).and_then(|value| value.parse().ok())
}

fn commands_from_attr(element: &Element) -> Vec<CanvasCommand> {
    element
        .attrs
        .get("data-agent-canvas-commands")
        .map(|raw| raw.split(';').filter_map(parse_command).collect())
        .unwrap_or_default()
}

fn parse_command(raw: &str) -> Option<CanvasCommand> {
    if raw.is_empty() {
        return None;
    }
    let parts = raw.split('|').collect::<Vec<_>>();
    let operation = parts.first()?.to_string();
    let args = parts
        .iter()
        .skip(1)
        .take(4)
        .filter_map(|value| value.parse().ok())
        .collect::<Vec<_>>();
    let style = parts.get(5).map(|value| value.to_string());
    Some(CanvasCommand {
        operation,
        args,
        style,
    })
}
