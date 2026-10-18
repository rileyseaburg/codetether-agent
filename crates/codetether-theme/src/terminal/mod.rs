//! Terminal capability detection and theme adaptation.
//!
//! [detect_color_support] inspects terminal hints. [ColorSupport] describes
//! available representations; [validate_theme] clones a theme and adapts its
//! colors to those capabilities without altering the caller's configuration.
mod detect;
mod fallback;
mod indexed;
mod rgb_indexed;
mod rgb_named;
mod support;
mod validate;
pub use detect::detect_color_support;
pub use support::ColorSupport;
pub use validate::validate_theme;
#[cfg(test)]
mod tests;
