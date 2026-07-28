//! Canvas surface painting loops.

use super::surface::Surface;

pub(super) fn paint(surface: &mut Surface, rect: (i64, i64, i64, i64), color: [u8; 4]) {
    let Some((x0, y0, x1, y1)) = super::geometry::clip(rect, surface.width, surface.height) else {
        return;
    };
    if surface.pixels.is_empty() {
        return;
    }
    for y in y0..y1 {
        for x in x0..x1 {
            surface.pixels[y * surface.width as usize + x] = color;
        }
    }
}
