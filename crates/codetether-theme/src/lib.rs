//! Serializable themes and presets, independent of any UI runtime.
//!
//! Enable the `ratatui` feature for terminal colors and conversation styles.
//! Configuration consumers can use the data types without a renderer.
//!
//! # Examples
//! ```
//! use codetether_theme::{ColorDef, Theme};
//! assert_eq!(Theme::dark().assistant_color, ColorDef::cyan());
//! ```

mod color;
#[cfg(feature = "ratatui")]
mod color_from;
mod color_named;
#[cfg(feature = "ratatui")]
mod color_render;
mod dark;
mod default;
mod light;
mod marketing;
#[cfg(feature = "ratatui")]
mod roles;
mod solarized_dark;
mod solarized_light;
mod theme;
pub use color::ColorDef;
pub use theme::Theme;
