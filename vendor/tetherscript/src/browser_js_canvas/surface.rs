//! Canvas pixel surface and command log.

use super::color;

pub(super) const MAX_PIXELS: usize = 1_000_000;

#[derive(Clone)]
pub(super) struct Surface {
    pub width: u32,
    pub height: u32,
    pub commands: Vec<String>,
    pub pixels: Vec<[u8; 4]>,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width as usize).saturating_mul(height as usize);
        let pixels = if len <= MAX_PIXELS {
            vec![[0, 0, 0, 0]; len]
        } else {
            Vec::new()
        };
        Self {
            width,
            height,
            commands: Vec::new(),
            pixels,
        }
    }

    pub fn fill_rect(&mut self, rect: (i64, i64, i64, i64), style: &str) {
        self.commands.push(format!(
            "fillRect|{}|{}|{}|{}|{}",
            rect.0,
            rect.1,
            rect.2,
            rect.3,
            color::safe_style(style)
        ));
        super::surface_paint::paint(self, rect, color::parse(style));
    }

    pub fn clear_rect(&mut self, rect: (i64, i64, i64, i64)) {
        self.commands.push(format!(
            "clearRect|{}|{}|{}|{}",
            rect.0, rect.1, rect.2, rect.3
        ));
        super::surface_paint::paint(self, rect, [0, 0, 0, 0]);
    }
}
