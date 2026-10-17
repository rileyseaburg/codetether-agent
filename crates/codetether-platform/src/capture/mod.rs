//! Checked pixel-buffer sizing and portable BGRA image encoding.
//!
//! [pixel_bytes] rejects invalid or excessive captures before allocation.
//! [bgra_to_png] and [bgra_to_encoded] normalize GDI's reserved alpha channel
//! and encode the original capture dimensions.
mod encode;
mod limits;
pub use encode::{bgra_to_encoded, bgra_to_png};
pub use limits::{MAX_PIXELS, pixel_bytes};
