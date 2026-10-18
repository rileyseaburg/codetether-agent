//! Image data-URL ingestion for an interactive text composer.
//!
//! [try_attach_data_url] consumes standalone images and reports rejections.
//! [drain_embedded_images] extracts images from composer text through [State].
mod attach;
mod drain;
mod state;
pub use attach::try_attach_data_url;
pub use drain::drain_embedded_images;
pub use state::State;
