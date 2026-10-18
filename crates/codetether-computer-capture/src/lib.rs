//! Bounded computer-use captures with original-coordinate metadata.
//!
//! [limits] validates capture and OCR input sizes. [prepare] retains a small
//! PNG or produces a bounded JPEG preview; [Preview] carries the attachment
//! and scale factors needed to map preview coordinates to original pixels.
pub mod limits;
mod preview;
pub use preview::{Preview, prepare};
