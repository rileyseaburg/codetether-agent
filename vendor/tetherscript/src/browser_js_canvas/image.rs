//! Canvas image-data snapshots and checksums.

use super::surface::{Surface, MAX_PIXELS};
use super::*;

pub(super) fn image_data(handle: &DomHandle, rect: (i64, i64, i64, i64)) -> JsValue {
    super::store::with_surface(handle, |surface| image_data_object(surface, rect))
}

pub(super) fn summary(handle: &DomHandle) -> String {
    super::store::with_surface(handle, |surface| {
        format!(
            "{}x{}:{}:{}",
            surface.width,
            surface.height,
            surface.commands.len(),
            super::pixels::checksum(surface)
        )
    })
}

pub(super) fn sync_attrs(handle: &DomHandle, surface: &Surface) {
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            el.attrs
                .insert("data-agent-canvas-width".into(), surface.width.to_string());
            el.attrs.insert(
                "data-agent-canvas-height".into(),
                surface.height.to_string(),
            );
            el.attrs.insert(
                "data-agent-canvas-commands".into(),
                surface.commands.join(";"),
            );
            el.attrs.insert(
                "data-agent-canvas-checksum".into(),
                super::pixels::checksum(surface).to_string(),
            );
        }
    });
}

fn image_data_object(surface: &Surface, rect: (i64, i64, i64, i64)) -> JsValue {
    let width = rect.2.max(0) as usize;
    let height = rect.3.max(0) as usize;
    let mut bytes = Vec::new();
    if width.saturating_mul(height) <= MAX_PIXELS {
        for row in 0..height {
            for col in 0..width {
                let pixel = super::pixels::at(surface, rect.0 + col as i64, rect.1 + row as i64);
                bytes.extend(pixel.into_iter().map(|b| JsValue::Number(b as f64)));
            }
        }
    }
    super::pixels::object(width, height, bytes, super::pixels::checksum(surface))
}
