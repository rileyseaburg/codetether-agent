//! WebGL state serialization onto canvas attributes.

use super::{webgl_state::WebGlState, *};

pub(super) fn sync_attrs(handle: &DomHandle, state: &WebGlState) {
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            el.attrs
                .insert("data-agent-webgl-version".into(), state.version.to_string());
            el.attrs
                .insert("data-agent-webgl-width".into(), state.width.to_string());
            el.attrs
                .insert("data-agent-webgl-height".into(), state.height.to_string());
            el.attrs.insert(
                "data-agent-webgl-viewport".into(),
                join_i64(&state.viewport),
            );
            el.attrs.insert(
                "data-agent-webgl-clear-color".into(),
                join_f64(&state.clear_color),
            );
            el.attrs
                .insert("data-agent-webgl-commands".into(), state.commands.join(";"));
            el.attrs.insert(
                "data-agent-webgl-extensions".into(),
                super::webgl_ext::SUPPORTED.join(";"),
            );
        }
    });
}

fn join_i64(values: &[i64; 4]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn join_f64(values: &[f64; 4]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
