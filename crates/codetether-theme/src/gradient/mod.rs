//! Terminal gradient colors and cached RGB capability detection.
//! [lerp_rgb] blends colors and [gradient_spans] styles individual characters.
//! [rgb_supported] supplies a single cached terminal capability decision.
mod interpolate;
mod spans;
mod support;
pub use interpolate::lerp_rgb;
pub use spans::gradient_spans;
pub use support::rgb_supported;

/// An RGB gradient endpoint.
pub type Rgb = (u8, u8, u8);

/// Neon cyan brand endpoint.
pub const NEON_CYAN: Rgb = (0, 229, 255);

/// Neon magenta brand endpoint.
pub const NEON_MAGENTA: Rgb = (255, 64, 216);
