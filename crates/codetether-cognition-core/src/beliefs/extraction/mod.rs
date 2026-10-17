//! Prompt construction and response decoding for belief extraction.
//!
//! Use [extraction_prompts] before invoking a model, then decode its text with
//! [parse_extracted_beliefs]. This module performs no provider calls.
mod decode;
mod prompts;
mod response;
pub use decode::parse_extracted_beliefs;
pub use prompts::extraction_prompts;
