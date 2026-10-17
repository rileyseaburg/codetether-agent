//! Session-scoped question/answer interaction inspired by OpenCode's question tool.
//!
//! [`Request`] holds a batch of questions. [`Draft`] provides a UI-independent
//! selection state machine. [`runtime`] connects async tools to the frontend
//! without reading stdin. Frontends opt in using [`runtime::scope`]; spawned
//! children and headless tasks do not inherit the interactive capability.
//!
//! ```
//! use codetether_question::{Request, Draft, Action};
//! let request = Request::parse(serde_json::json!({"questions": [{
//!     "question": "Which mode?", "header": "Mode",
//!     "options": [{"label": "Fast", "description": "Lower latency"}]
//! }]})).unwrap();
//! let mut draft = Draft::new(request);
//! assert_eq!(draft.apply(Action::Submit), Some(vec![vec!["Fast".to_owned()]]));
//! ```

mod draft;
mod error;
mod request;
pub mod runtime;
mod schema;

pub use draft::{Action, Draft};
pub use error::Error;
pub use request::{Answers, Choice, Question, Request};
pub use schema::parameters;

#[cfg(test)]
mod tests;
