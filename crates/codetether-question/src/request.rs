//! Question wire contracts, compatibility decoding, and validation.
//!
//! [Request::parse] validates each batch; [Request::questions] exposes the
//! result. [Answers] preserves question order and selected label order.
mod legacy;
mod model;
mod parse;
mod validate;
mod view;
pub use model::{Answers, Choice, Question, Request};
