//! Native encoding and decoding for Gemini Web image conversations.
//!
//! [`build_payload`] produces the JSON value for the HTTP `f.req` form field;
//! [`model_header`] selects the observed image mode. [`parse_response`] decodes
//! the complete length-framed response and returns raw structured image URLs.
//! Transport owns authentication, URL policy, rendition selection and downloads.
//! Keep [`Continuation`] private when persisting it, and never log signed URLs.
//!
//! # Examples
//! ```
//! use codetether_gemini_web_protocol::images::{build_payload, model_header};
//! let payload = build_payload("Draw a mountain lake", None).unwrap();
//! assert!(payload.starts_with("[null,"));
//! assert!(model_header("test-build").contains("test-build"));
//! ```

mod attachments;
mod continuation;
mod framing;
mod header;
mod length;
mod parse;
mod path;
mod payload;
mod response;
mod status;
pub use continuation::Continuation;
pub use header::model_header;
pub use parse::parse_response;
pub use payload::build_payload;
pub use response::ImageResponse;
/// Observed image-capable mode selector; this does not attest a backend version.
pub const IMAGE_MODEL_ID: &str = "e051ce1aa80aa576";
