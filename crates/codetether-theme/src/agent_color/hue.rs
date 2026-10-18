//! Deterministic name hashing and full-saturation hue conversion.
use ratatui::style::Color;

pub(super) fn name_hue(name: &str) -> f32 {
    // FNV-1a fold to a float hue.
    let mut h: u32 = 2_166_136_261;
    for b in name.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(16_777_619);
    }
    (h % 360) as f32
}

pub(super) fn hue_to_rgb(hue: f32) -> Color {
    let h = (hue % 360.0) / 60.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as u32 {
        0 => (1.0_f32, x, 0.0_f32),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    let s = |v: f32| (v * 255.0).round() as u8;
    Color::Rgb(s(r), s(g), s(b))
}
