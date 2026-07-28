//! Text measurement using fixed-width approximation.

use std::collections::HashMap;

/// Approximate text width using a 0.6× font-size per character ratio.
pub fn measure_text_width(text: &str, font_size: i64) -> i64 {
    let chars = text.chars().count() as i64;
    (chars * font_size * 6 + 5) / 10
}

/// Measure line-height from CSS styles. Falls back to 1.2× font-size.
pub fn measure_line_height(styles: &HashMap<String, String>) -> i64 {
    if let Some(value) = styles.get("line-height") {
        if let Some(px) = parse_px(value) {
            return px;
        }
        if let Ok(multiplier) = value.parse::<f64>() {
            return ((font_size(styles) as f64) * multiplier).round() as i64;
        }
    }
    font_size(styles) * 6 / 5
}

fn font_size(styles: &HashMap<String, String>) -> i64 {
    styles
        .get("font-size")
        .and_then(|v| parse_px(v))
        .unwrap_or(16)
}

fn parse_px(value: &str) -> Option<i64> {
    value.trim().trim_end_matches("px").parse().ok()
}
