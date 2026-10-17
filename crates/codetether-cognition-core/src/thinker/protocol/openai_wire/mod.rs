//! OpenAI-compatible chat wire types for the HTTP thinker backend.
//!
//! Request/response shapes only; transport and retry logic stay in the client.

mod content;
mod request;
mod response;

pub use request::{OpenAIChatRequest, OpenAIMessage};
pub use response::OpenAIChatResponse;
