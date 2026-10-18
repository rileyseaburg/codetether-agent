//! Choose the most expressive supported representation for a color.
use super::{
    ColorSupport, indexed::indexed_to_named, rgb_indexed::rgb_to_ansi256, rgb_named::rgb_to_named,
};
use crate::ColorDef;

/// Convert colors to fallback based on terminal support
pub(super) fn fallback_color(color: &ColorDef, support: &ColorSupport) -> ColorDef {
    match color {
        ColorDef::Rgb(r, g, b) => {
            if support.supports_rgb() {
                ColorDef::Rgb(*r, *g, *b)
            } else if support.supports_indexed() {
                // Convert to closest indexed color
                ColorDef::Indexed(rgb_to_ansi256(*r, *g, *b))
            } else {
                // Convert to closest named color
                ColorDef::Named(rgb_to_named(*r, *g, *b))
            }
        }
        ColorDef::Indexed(idx) => {
            if support.supports_indexed() {
                ColorDef::Indexed(*idx)
            } else {
                // Convert to named color
                ColorDef::Named(indexed_to_named(*idx))
            }
        }
        ColorDef::Named(name) => {
            if support.supports_named() {
                ColorDef::Named(name.clone())
            } else {
                // Fallback to safe defaults
                ColorDef::Named("white".to_string())
            }
        }
    }
}
